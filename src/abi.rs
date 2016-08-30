#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum ClientRequest {
  RunningOnValgrind   = 0x1001,
  DiscardTranslations = 0x1002,
  // These allow any function to be called from the simulated CPU but run on the real CPU.
  // Nb: the first arg passed to the function is always the ThreadId of the running thread
  // So CLIENT_CALL0 actually requires a 1 arg function, etc.
  ClientCall0         = 0x1101,
  ClientCall1         = 0x1102,
  ClientCall2         = 0x1103,
  ClientCall3         = 0x1104,
  // Can be useful in regression testing suites -- eg. can send Valgrind's output to /dev/null and still count errors.
  CountErrors         = 0x1201,
  // Allows the client program and/or gdbserver to execute a monitor command.
  GdbMonitorCommand   = 0x1202,
  // These are useful and can be interpreted by any tool that tracks malloc() et al, by using vg_replace_malloc.c.
  MallocLikeBlock     = 0x1301,
  ResizeInPlaceBlock  = 0x130b,
  FreeLikeBlock       = 0x1302,
  /* Memory pool support. */
  CreateMemPool       = 0x1303,
  DestroyMemPool      = 0x1304,
  MemPoolAlloc        = 0x1305,
  MemPoolFree         = 0x1306,
  MemPoolTrim         = 0x1307,
  MoveMemPool         = 0x1308,
  MemPoolChange       = 0x1309,
  MemPoolExists       = 0x130a,
  // Allow printfs to valgrind log.
  // The first two pass the va_list argument by value, which assumes it is the same size as or smaller than a UWord,
  // which generally isn't the case.  Hence are deprecated.
  // The second two pass the vargs by reference and so are immune to this problem.
  // both :: char* fmt, va_list vargs (DEPRECATED)
  Printf          = 0x1401,
  PrintfBacktrace = 0x1402,
  // both :: char* fmt, va_list* vargs
  PrintfVaListByRef          = 0x1403,
  PrintfBacktraceVaListByRef = 0x1404,
  // Stack support.
  StackRegister   = 0x1501,
  StackDeregister = 0x1502,
  StackChange     = 0x1503,
  // Wine support
  LoadPdbDebugInfo = 0x1601,
  // Querying of debug info.
  MapIpToSourceLoc = 0x1701,
  // Disable/enable error reporting level.
  // Takes a single Word arg which is the delta to this thread's error disablement indicator.
  // Hence 1 disables or further disables errors, and -1 moves back towards enablement. Other values are not allowed.
  ChangeErrDisablement = 0x1801,
  // Initialise IR injection
  VexInitForIri = 0x1901
}
