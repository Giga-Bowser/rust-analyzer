searchState.loadedDescShard("salsa", 0, "The salsa crate is a crate for incremental recomputation.  …\nA panic payload indicating that execution of a salsa query …\nCaptures the participants of a cycle that occurred when …\nThe base trait which your “query context” must …\nAn integer that uniquely identifies a particular query …\nOccurs when we found that all inputs to a memoized value …\nDescribes how likely a value is to change – how “…\nDyn version of the associated trait for this query group.\nThe <code>Event</code> struct identifies various notable things that can\nAn enum identifying the various kinds of events that can …\nAssociate query group struct.\nGenerated struct that contains storage for all queries in …\nHigh durability: things that are not expected to change …\nThe “raw-id” is used for interned keys in salsa – it …\nTrait implemented for the “key” that results from a …\nTrait implemented for the “value” that is being …\nThey key used to intern this value by.\nType that you give as a parameter – for queries with zero\nLow durability: things that change frequently.\nThe maximum allowed <code>InternId</code>. This value can grow between …\nMedium durability: things that change sometimes, but …\nIndicates a database that also supports parallel query …\nThe query was operating on revision R, but there is a …\nThe query was blocked on another thread, and that thread …\nA unique index identifying this query within the group.\nName of the query method (e.g., <code>foo</code>)\nTrait implements by all of the “special types” …\nTrait implements by all of the “special types” …\nReturn value from the <code>query</code> method on <code>Database</code>. Gives …\nReturn value from the <code>query_mut</code> method on <code>Database</code>. Gives …\nA unique identifier for the current version of the …\nThe salsa runtime stores the storage for all queries as …\nA unique identifier for a particular runtime. Each time …\nSimple wrapper struct that takes ownership of a database <code>DB</code>…\nStores the cached results and dependency information for …\nInternal struct storing the values for the query.\nWhat value does the query return?\nIndicates that another thread (with id <code>other_runtime_id</code>) …\nIndicates that <code>unwind_if_cancelled</code> was called and salsa …\nIndicates that the function for this query will be …\nReturns the database-key for the query that this thread is …\nReturns a vector with the debug information for all the …\nExtract the <code>u32</code> with which the intern-key was created.\nConvert this raw-id into a u32 value.\nConvert this raw-id into a usize value.\nRuns <code>f</code>, and catches any salsa cancellation.\nThis attribute is placed on your database struct. It takes …\nDebugging APIs: these are meant for use when unit-testing …\nReturns a type that gives a user-readable debug output. …\nReturns a type that gives a user-readable debug output. …\nReturns a type that gives a user-readable debug output. …\nReturns a “debug” view onto this strict that can be …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreate an instance of the intern-key from a <code>u32</code> value.\nExecute the query on a given input. Usually it’s easier …\nFetches the intern id for the given key or inserts it if …\nReturns the index of the query group containing this key.\nThe unique identifier attached to this <code>SalsaRuntime</code>. Each …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMaps the value to a key that will be used to intern it.\nMarks the computed value as outdated.\nReturns the index of this particular query key within the …\nWhat sort of event was it.\nCreate a new runtime; equivalent to <code>Self::default</code>. This is …\nConstructs a new <code>QueryTable</code>.\nConstructs a new <code>QueryTableMut</code>.\nCreates a <code>Snapshot</code> that wraps the given database handle <code>db</code>…\nCreates a new InternId.\nIterate over the <code>DatabaseKeyIndex</code> for each query …\nCompletely clears the storage for this query.\nThe decorator that defines a salsa “query group” …\nReturns the index of the query within its query group.\nExtract storage for this query from the storage for its …\nExtract storage for this query from the storage for its …\nAccess the query storage tables. Not meant to be used …\nAccess the query storage tables. Not meant to be used …\nActs as though the current query had read an input with …\nReports that the query depends on some state unknown to …\nThe id of the snapshot that triggered the event.  Usually …\nThis function is invoked at key points in the salsa …\nGives access to the underlying salsa runtime.\nGives access to the underlying salsa runtime.\nGives access to the underlying salsa runtime.\nAssign a value to an “input query”. Must be used …\nSets the size of LRU cache of values for this query table.\nAssign a value to an “input query”, with the additional\nCreates a second handle to the database that holds the …\nReturns a “snapshotted” storage, suitable for use in a …\nA “synthetic write” causes the system to act <em>as though</em> …\nA “synthetic write” causes the system to act <em>as though</em> …\nReturns a vector with the debug information for those …\nStarts unwinding the stack if the current revision is …\nCalls the given function with the key that was used to …\nCalls the given function with the key that was used to …\nThe database-key for the affected value. Implements <code>Debug</code>.\nThe database-key for the affected value. Implements <code>Debug</code>.\nThe database-key for the affected value. Implements <code>Debug</code>.\nThe id of the runtime we will block on.\nAdditional methods on queries that can be used to “peek …\nKey of this query.\nAn entry from a query table, for debugging and inspecting …\nValue of this query.\nReturns a lower bound on the durability for the given key. …\nGet the (current) set of the entries in the query table.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nkey of the query\nvalue of the query, if it is stored")