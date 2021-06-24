use hir::Semantics;
use ide_db::{
    base_db::FilePosition,
    defs::Definition,
    helpers::pick_best_token,
    search::{FileReference, ReferenceAccess, SearchScope},
    RootDatabase,
};
use syntax::{ast, match_ast, AstNode, SyntaxNode, SyntaxToken, TextRange, WalkEvent, T};

use crate::{display::TryToNav, references, NavigationTarget};

pub struct HighlightedRange {
    pub range: TextRange,
    pub access: Option<ReferenceAccess>,
}

// Feature: Highlight Related
//
// Highlights constructs related to the thing under the cursor:
// - if on an identifier, highlights all references to that identifier in the current file
// - if on an `async` or `await token, highlights all yield points for that async context
// - if on a `return` token, `?` character or `->` return type arrow, highlights all exit points for that context
pub(crate) fn highlight_related(
    sema: &Semantics<RootDatabase>,
    position: FilePosition,
) -> Option<Vec<HighlightedRange>> {
    let _p = profile::span("highlight_related");
    let syntax = sema.parse(position.file_id).syntax().clone();

    let token = pick_best_token(syntax.token_at_offset(position.offset), |kind| match kind {
        T![?] => 2, // prefer `?` when the cursor is sandwiched like `await$0?`
        T![await] | T![async] | T![return] | T![->] => 1,
        _ => 0,
    })?;

    match token.kind() {
        T![return] | T![?] | T![->] => highlight_exit_points(sema, token),
        T![await] | T![async] => highlight_yield_points(token),
        _ => highlight_references(sema, &syntax, position),
    }
}

fn highlight_references(
    sema: &Semantics<RootDatabase>,
    syntax: &SyntaxNode,
    FilePosition { offset, file_id }: FilePosition,
) -> Option<Vec<HighlightedRange>> {
    let def = references::find_def(sema, syntax, offset)?;
    let usages = def.usages(sema).set_scope(Some(SearchScope::single_file(file_id))).all();

    let declaration = match def {
        Definition::ModuleDef(hir::ModuleDef::Module(module)) => {
            Some(NavigationTarget::from_module_to_decl(sema.db, module))
        }
        def => def.try_to_nav(sema.db),
    }
    .filter(|decl| decl.file_id == file_id)
    .and_then(|decl| {
        let range = decl.focus_range?;
        let access = references::decl_access(&def, syntax, range);
        Some(HighlightedRange { range, access })
    });

    let file_refs = usages.references.get(&file_id).map_or(&[][..], Vec::as_slice);
    let mut res = Vec::with_capacity(file_refs.len() + 1);
    res.extend(declaration);
    res.extend(
        file_refs
            .iter()
            .map(|&FileReference { access, range, .. }| HighlightedRange { range, access }),
    );
    Some(res)
}

fn highlight_exit_points(
    sema: &Semantics<RootDatabase>,
    token: SyntaxToken,
) -> Option<Vec<HighlightedRange>> {
    fn hl(
        sema: &Semantics<RootDatabase>,
        body: Option<ast::Expr>,
    ) -> Option<Vec<HighlightedRange>> {
        let mut highlights = Vec::new();
        let body = body?;
        walk(&body, &mut |expr| {
            match expr {
                ast::Expr::ReturnExpr(expr) => {
                    if let Some(token) = expr.return_token() {
                        highlights
                            .push(HighlightedRange { access: None, range: token.text_range() });
                    }
                }
                ast::Expr::TryExpr(try_) => {
                    if let Some(token) = try_.question_mark_token() {
                        highlights
                            .push(HighlightedRange { access: None, range: token.text_range() });
                    }
                }
                ast::Expr::MethodCallExpr(_) | ast::Expr::CallExpr(_) | ast::Expr::MacroCall(_) => {
                    if sema.type_of_expr(&expr).map_or(false, |ty| ty.is_never()) {
                        highlights.push(HighlightedRange {
                            access: None,
                            range: expr.syntax().text_range(),
                        });
                    }
                }
                ast::Expr::EffectExpr(effect) => {
                    return effect.async_token().is_some() || effect.try_token().is_some()
                }
                ast::Expr::ClosureExpr(_) => return true,
                _ => (),
            }
            false
        });
        let tail = match body {
            ast::Expr::BlockExpr(b) => b.tail_expr(),
            e => Some(e),
        };

        if let Some(tail) = tail {
            highlights.push(HighlightedRange { access: None, range: tail.syntax().text_range() })
        }
        Some(highlights)
    }
    for anc in token.ancestors() {
        return match_ast! {
            match anc {
                ast::Fn(fn_) => hl(sema, fn_.body().map(ast::Expr::BlockExpr)),
                ast::ClosureExpr(closure) => hl(sema, closure.body()),
                ast::EffectExpr(effect) => if matches!(effect.effect(), ast::Effect::Async(_) | ast::Effect::Try(_)| ast::Effect::Const(_)) {
                    hl(sema, effect.block_expr().map(ast::Expr::BlockExpr))
                } else {
                    continue;
                },
                _ => continue,
            }
        };
    }
    None
}

fn highlight_yield_points(token: SyntaxToken) -> Option<Vec<HighlightedRange>> {
    fn hl(
        async_token: Option<SyntaxToken>,
        body: Option<ast::Expr>,
    ) -> Option<Vec<HighlightedRange>> {
        let mut highlights = Vec::new();
        highlights.push(HighlightedRange { access: None, range: async_token?.text_range() });
        if let Some(body) = body {
            walk(&body, &mut |expr| {
                match expr {
                    ast::Expr::AwaitExpr(expr) => {
                        if let Some(token) = expr.await_token() {
                            highlights
                                .push(HighlightedRange { access: None, range: token.text_range() });
                        }
                    }
                    // All the following are different contexts so skip them
                    ast::Expr::EffectExpr(effect) => {
                        return matches!(
                            effect.effect(),
                            ast::Effect::Async(_) | ast::Effect::Try(_) | ast::Effect::Const(_)
                        )
                    }
                    ast::Expr::ClosureExpr(__) => return true,
                    _ => (),
                }
                false
            });
        }
        Some(highlights)
    }
    for anc in token.ancestors() {
        return match_ast! {
            match anc {
                ast::Fn(fn_) => hl(fn_.async_token(), fn_.body().map(ast::Expr::BlockExpr)),
                ast::EffectExpr(effect) => hl(effect.async_token(), effect.block_expr().map(ast::Expr::BlockExpr)),
                ast::ClosureExpr(closure) => hl(closure.async_token(), closure.body()),
                _ => continue,
            }
        };
    }
    None
}

/// Preorder walk the expression node skipping a node's subtrees if the callback returns `true` for the node.
fn walk(expr: &ast::Expr, cb: &mut dyn FnMut(ast::Expr) -> bool) {
    let mut preorder = expr.syntax().preorder();
    while let Some(event) = preorder.next() {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(_) => continue,
        };
        match ast::Stmt::cast(node.clone()) {
            Some(ast::Stmt::LetStmt(l)) => {
                if let Some(expr) = l.initializer() {
                    walk(&expr, cb);
                }
            }
            // Don't skip subtree since we want to process the expression behind this next
            Some(ast::Stmt::ExprStmt(_)) => continue,
            // skip inner items which might have their own expressions
            Some(ast::Stmt::Item(_)) => (),
            None => {
                if let Some(expr) = ast::Expr::cast(node) {
                    if !cb(expr) {
                        continue;
                    }
                }
            }
        }
        preorder.skip_subtree();
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture;

    use super::*;

    fn check(ra_fixture: &str) {
        let (analysis, pos, annotations) = fixture::annotations(ra_fixture);
        let hls = analysis.highlight_related(pos).unwrap().unwrap();

        let mut expected = annotations
            .into_iter()
            .map(|(r, access)| (r.range, (!access.is_empty()).then(|| access)))
            .collect::<Vec<_>>();

        let mut actual = hls
            .into_iter()
            .map(|hl| {
                (
                    hl.range,
                    hl.access.map(|it| {
                        match it {
                            ReferenceAccess::Read => "read",
                            ReferenceAccess::Write => "write",
                        }
                        .to_string()
                    }),
                )
            })
            .collect::<Vec<_>>();
        actual.sort_by_key(|(range, _)| range.start());
        expected.sort_by_key(|(range, _)| range.start());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hl_module() {
        check(
            r#"
//- /lib.rs
mod foo$0;
 // ^^^
//- /foo.rs
struct Foo;
"#,
        );
    }

    #[test]
    fn test_hl_self_in_crate_root() {
        check(
            r#"
use self$0;
"#,
        );
    }

    #[test]
    fn test_hl_self_in_module() {
        check(
            r#"
//- /lib.rs
mod foo;
//- /foo.rs
use self$0;
"#,
        );
    }

    #[test]
    fn test_hl_local() {
        check(
            r#"
fn foo() {
    let mut bar = 3;
         // ^^^ write
    bar$0;
 // ^^^ read
}
"#,
        );
    }

    #[test]
    fn test_hl_yield_points() {
        check(
            r#"
pub async fn foo() {
 // ^^^^^
    let x = foo()
        .await$0
      // ^^^^^
        .await;
      // ^^^^^
    || { 0.await };
    (async { 0.await }).await
                     // ^^^^^
}
"#,
        );
    }

    #[test]
    fn test_hl_yield_points2() {
        check(
            r#"
pub async$0 fn foo() {
 // ^^^^^
    let x = foo()
        .await
      // ^^^^^
        .await;
      // ^^^^^
    || { 0.await };
    (async { 0.await }).await
                     // ^^^^^
}
"#,
        );
    }

    #[test]
    fn test_hl_yield_nested_fn() {
        check(
            r#"
async fn foo() {
    async fn foo2() {
 // ^^^^^
        async fn foo3() {
            0.await
        }
        0.await$0
       // ^^^^^
    }
    0.await
}
"#,
        );
    }

    #[test]
    fn test_hl_yield_nested_async_blocks() {
        check(
            r#"
async fn foo() {
    (async {
  // ^^^^^
        (async {
           0.await
        }).await$0 }
        // ^^^^^
    ).await;
}
"#,
        );
    }

    #[test]
    fn test_hl_exit_points() {
        check(
            r#"
fn foo() -> u32 {
    if true {
        return$0 0;
     // ^^^^^^
    }

    0?;
  // ^
    0xDEAD_BEEF
 // ^^^^^^^^^^^
}
"#,
        );
    }

    #[test]
    fn test_hl_exit_points2() {
        check(
            r#"
fn foo() ->$0 u32 {
    if true {
        return 0;
     // ^^^^^^
    }

    0?;
  // ^
    0xDEAD_BEEF
 // ^^^^^^^^^^^
}
"#,
        );
    }

    #[test]
    fn test_hl_prefer_ref_over_tail_exit() {
        check(
            r#"
fn foo() -> u32 {
// ^^^
    if true {
        return 0;
    }

    0?;

    foo$0()
 // ^^^
}
"#,
        );
    }

    #[test]
    fn test_hl_never_call_is_exit_point() {
        check(
            r#"
struct Never;
impl Never {
    fn never(self) -> ! { loop {} }
}
macro_rules! never {
    () => { never() }
}
fn never() -> ! { loop {} }
fn foo() ->$0 u32 {
    never();
 // ^^^^^^^
    never!();
 // FIXME sema doesn't give us types for macrocalls

    Never.never();
 // ^^^^^^^^^^^^^

    0
 // ^
}
"#,
        );
    }
}
