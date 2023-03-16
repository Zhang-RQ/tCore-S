#![allow(unused)]

pub const ALL_TESTCASES: &[&str] = &[
    "busybox sh busybox_testcode.sh",
    "busybox sh lua_testcode.sh",
    "busybox sh lmbench_testcode.sh",
];

pub const BUSYBOX_TESTCASES: &[&str] = &[
    "busybox echo \"#### independent command test\"",
    "busybox ash -c exit",
    "busybox sh -c exit",
    "busybox basename /aaa/bbb",
    "busybox cal",
    "busybox clear",
    "busybox date",
    "busybox df",
    "busybox dirname /aaa/bbb",
    "busybox dmesg",
    "busybox du",
    "busybox expr 1 + 1",
    "busybox false",
    "busybox true",
    "busybox which ls",
    "busybox uname",
    "busybox uptime",
    "busybox printf \"abc\n\"",
    "busybox ps",
    "busybox pwd",
    "busybox free",
    "busybox hwclock",
    "busybox kill 10",
    "busybox ls",
    "busybox sleep 1",
    "busybox echo \"#### file opration test\"",
    "busybox touch test.txt",
    "busybox echo \"hello world\" > test.txt",
    "busybox cat test.txt",
    "busybox cut -c 3 test.txt",
    "busybox od test.txt",
    "busybox head test.txt",
    "busybox tail test.txt",
    "busybox md5sum test.txt",
    "busybox echo \"ccccccc\" >> test.txt",
    "busybox echo \"bbbbbbb\" >> test.txt",
    "busybox echo \"aaaaaaa\" >> test.txt",
    "busybox echo \"2222222\" >> test.txt",
    "busybox echo \"1111111\" >> test.txt",
    "busybox echo \"bbbbbbb\" >> test.txt",
    "busybox sort test.txt | ./busybox uniq",
    "busybox stat test.txt",
    "busybox strings test.txt",
    "busybox wc test.txt",
    "busybox [ -f test.txt ]",
    "busybox more test.txt",
    "busybox rm test.txt",
    "busybox mkdir test_dir",
    "busybox mv test_dir test",
    "busybox rmdir test",
    "busybox grep hello busybox_cmd.txt",
    "busybox cp busybox_cmd.txt busybox_cmd.bak",
    "busybox rm busybox_cmd.bak",
    "busybox find -name \"busybox_cmd.txt\"",
];

pub const LUA_TESTCASES: &[&str] = &[
    "lua date.lua",
    "lua file_io.lua",
    "lua max_min.lua",
    "lua random.lua",
    "lua remove.lua",
    "lua round_num.lua",
    "lua sin30.lua",
    "lua sort.lua",
    "lua strings.lua",
];

pub const LIBC_STATIC_TESTCASES: &[&str] = &[
    "argv",
    "basename",
    "clock_gettime",
    "clocale_mbfuncs",
    "crypt",
    "dirname",
    "env",
    "fdopen",
    "fnmatch",
    "fscanf",
    "iconv_open",
    "inet_pton",
    "mbc",
    "memstream",
    "pthread_cancel_points",
    "pthread_cancel",
    "pthread_cond",
    "pthread_tsd",
    "qsort",
    "random",
    "search_hsearch",
    "search_insque",
    "search_lsearch",
    "search_tsearch",
    "setjmp",
    "snprintf",
    "socket",
    "sscanf",
    "sscanf_long",
    "stat",
    "strftime",
    "string",
    "string_memcpy",
    "string_memmem",
    "string_memset",
    "string_strchr",
    "string_strcspn",
    "string_strstr",
    "strptime",
    "strtod",
    "strtod_simple",
    "strtof",
    "strtol",
    "strtold",
    "swprintf",
    "tgmath",
    "time",
    "tls_algin",
    "udiv",
    "ungetc",
    "utime",
    "wcsstr",
    "wcstol",
    "pleval",
    "daemon_failure",
    "dn_expand_empty",
    "dn_expand_ptr_0",
    "fflush_exit",
    "fgets_eof",
    "fgetwc_buffering",
    "fpclassify_invalid_ld80",
    "ftello_unflushed_append",
    "getpwnam_r_crash",
    "getpwnam_r_errno",
    "iconv_roundtrips",
    "inet_ntop_v4mapped",
    "inet_pton_empty_last_field",
    "iswspace_null",
    "lrand48_signextend",
    "lseek_large",
    "malloc_0",
    "mbsrtowcs_overflow",
    "memmem_oob_read",
    "memmem_oob",
    "mkdtemp_failure",
    "mkstemp_failure",
    "printf_1e9_oob",
    "printf_fmt_g_round",
    "printf_fmt_g_zeros",
    "printf_fmt_n",
    "pthread_robust_detach",
    "pthread_cond_smasher",
    "pthread_condattr_setclock",
    "pthread_exit_cancel",
    "pthread_once_deadlock",
    "pthread_rwlock_ebusy",
    "putenv_doublefree",
    "regex_backref_0",
    "regex_bracket_icase",
    "regex_ere_backref",
    "regex_escaped_high_byte",
    "regex_negated_range",
    "regexec_nosub",
    "rewind_clear_error",
    "rlimit_open_files",
    "scanf_bytes_consumed",
    "scanf_match_literal_eof",
    "scanf_nullbyte_char",
    "setvbuf_unget",
    "sigprocmask_internal",
    "sscanf_eof",
    "stavfs",
    "strverscmp",
    "syscall_sign_extend",
    "tls_get_new_dtv",
    "uselocale_0",
    "wcsncpy_read_overflow",
    "wcsstr_false_negative",
];

pub const LIBC_DYNAMIC_TESTCASES: &[&str] = &[
    "dyn/getpwnam_r_crash.dout",
    "dyn/fflush_exit.dout",
    "dyn/tls_local_exec.dout",
    "dyn/inet_ntop_v4mapped.dout",
    "dyn/mkstemp_failure.dout",
    "dyn/utime.dout",
    "dyn/setjmp.dout",
    "dyn/search_tsearch.dout",
    "dyn/memmem_oob_read.dout",
    "dyn/mbc.dout",
    "dyn/string_memset.dout",
    "dyn/time.dout",
    "dyn/pthread_cond_smasher.dout",
    "dyn/fgetwc_buffering.dout",
    "dyn/pthread_rwlock_ebusy.dout",
    "dyn/sscanf_long.dout",
    "dyn/strptime.dout",
    "dyn/dn_expand_empty.dout",
    "dyn/wcsstr.dout",
    "dyn/basename.dout",
    "dyn/lrand48_signextend.dout",
    "dyn/regex_negated_range.dout",
    "dyn/sigprocmask_internal.dout",
    "dyn/string.dout",
    "dyn/pthread_cancel.dout",
    "dyn/crypt.dout",
    "dyn/search_hsearch.dout",
    "dyn/clocale_mbfuncs.dout",
    "dyn/regex_bracket_icase.dout",
    "dyn/snprintf.dout",
    "dyn/strverscmp.dout",
    "dyn/sem_init.dout",
    "dyn/random.dout",
    "dyn/strtold.dout",
    "dyn/iswspace_null.dout",
    "dyn/regex_ere_backref.dout",
    "dyn/tls_get_new_dtv.dout",
    "dyn/ftello_unflushed_append.dout",
    "dyn/pthread_tsd.dout",
    "dyn/pthread_exit_cancel.dout",
    "dyn/string_strchr.dout",
    "dyn/printf_fmt_g_zeros.dout",
    "dyn/daemon_failure.dout",
    "dyn/mbsrtowcs_overflow.dout",
    "dyn/strtod_simple.dout",
    "dyn/inet_pton_empty_last_field.dout",
    "dyn/strtol.dout",
    "dyn/fscanf.dout",
    "dyn/tgmath.dout",
    "dyn/ungetc.dout",
    "dyn/dn_expand_ptr_0.dout",
    "dyn/socket.dout",
    "dyn/wcsncpy_read_overflow.dout",
    "dyn/getpwnam_r_errno.dout",
    "dyn/argv.dout",
    "dyn/fpclassify_invalid_ld80.dout",
    "dyn/string_memcpy.dout",
    "dyn/setvbuf_unget.dout",
    "dyn/putenv_doublefree.dout",
    "dyn/pthread_cancel_points.dout",
    "dyn/search_insque.dout",
    "dyn/scanf_bytes_consumed.dout",
    "dyn/dirname.dout",
    "dyn/string_strcspn.dout",
    "dyn/clock_gettime.dout",
    "dyn/wcstol.dout",
    "dyn/fdopen.dout",
    "dyn/scanf_match_literal_eof.dout",
    "dyn/sscanf_eof.dout",
    "dyn/pthread_once_deadlock.dout",
    "dyn/fwscanf.dout",
    "dyn/env.dout",
    "dyn/mkdtemp_failure.dout",
    "dyn/fnmatch.dout",
    "dyn/strftime.dout",
    "dyn/wcsstr_false_negative.dout",
    "dyn/syscall_sign_extend.dout",
    "dyn/swprintf.dout",
    "dyn/tls_init.dout",
    "dyn/regexec_nosub.dout",
    "dyn/string_strstr.dout",
    "dyn/scanf_nullbyte_char.dout",
    "dyn/regex_escaped_high_byte.dout",
    "dyn/printf_fmt_g_round.dout",
    "dyn/pthread_cond.dout",
    "dyn/stat.dout",
    "dyn/sscanf.dout",
    "dyn/dlopen.dout",
    "dyn/printf_fmt_n.dout",
    "dyn/uselocale_0.dout",
    "dyn/regex_backref_0.dout",
    "dyn/qsort.dout",
    "dyn/pthread_condattr_setclock.dout",
    "dyn/inet_pton.dout",
    "dyn/search_lsearch.dout",
    "dyn/strtod.dout",
    "dyn/memmem_oob.dout",
    "dyn/string_memmem.dout",
    "dyn/fgets_eof.dout",
    "dyn/rlimit_open_files.dout",
    "dyn/strtof.dout",
    "dyn/memstream.dout",
    "dyn/udiv.dout",
    "dyn/malloc_0.dout",
    "dyn/printf_1e9_oob.dout",
    "dyn/pthread_robust_detach.dout",
    "dyn/rewind_clear_error.dout",
    "dyn/iconv_roundtrips.dout",
    "dyn/lseek_large.dout",
    "dyn/stavfs.dout",
    "dyn/iconv_open.dout",
];

pub const FORMAT_LIBC_STATIC: &[&str] = &[
    "./runtest.exe -w entry-static.exe argv",
    // "./runtest.exe -w entry-static.exe basename",
    // "./runtest.exe -w entry-static.exe clocale_mbfuncs",
    // "./runtest.exe -w entry-static.exe clock_gettime",
    // "./runtest.exe -w entry-static.exe crypt",
    // "./runtest.exe -w entry-static.exe dirname",
    // "./runtest.exe -w entry-static.exe env",
    // "./runtest.exe -w entry-static.exe fdopen",
    // "./runtest.exe -w entry-static.exe fnmatch",
    // "./runtest.exe -w entry-static.exe fscanf",
    // "./runtest.exe -w entry-static.exe fwscanf",
    // "./runtest.exe -w entry-static.exe iconv_open",
    // "./runtest.exe -w entry-static.exe inet_pton",
    // "./runtest.exe -w entry-static.exe mbc",
    // "./runtest.exe -w entry-static.exe memstream",
    // "./runtest.exe -w entry-static.exe pthread_cancel_points",
    // "./runtest.exe -w entry-static.exe pthread_cancel",
    // "./runtest.exe -w entry-static.exe pthread_cond",
    // "./runtest.exe -w entry-static.exe pthread_tsd",
    // "./runtest.exe -w entry-static.exe qsort",
    // "./runtest.exe -w entry-static.exe random",
    // "./runtest.exe -w entry-static.exe search_hsearch",
    // "./runtest.exe -w entry-static.exe search_insque",
    // "./runtest.exe -w entry-static.exe search_lsearch",
    // "./runtest.exe -w entry-static.exe search_tsearch",
    // "./runtest.exe -w entry-static.exe setjmp",
    // "./runtest.exe -w entry-static.exe snprintf",
    // "./runtest.exe -w entry-static.exe socket",
    // "./runtest.exe -w entry-static.exe sscanf",
    // "./runtest.exe -w entry-static.exe sscanf_long",
    // "./runtest.exe -w entry-static.exe stat",
    // "./runtest.exe -w entry-static.exe strftime",
    // "./runtest.exe -w entry-static.exe string",
    // "./runtest.exe -w entry-static.exe string_memcpy",
    // "./runtest.exe -w entry-static.exe string_memmem",
    // "./runtest.exe -w entry-static.exe string_memset",
    // "./runtest.exe -w entry-static.exe string_strchr",
    // "./runtest.exe -w entry-static.exe string_strcspn",
    // "./runtest.exe -w entry-static.exe string_strstr",
    // "./runtest.exe -w entry-static.exe strptime",
    // "./runtest.exe -w entry-static.exe strtod",
    // "./runtest.exe -w entry-static.exe strtod_simple",
    // "./runtest.exe -w entry-static.exe strtof",
    // "./runtest.exe -w entry-static.exe strtol",
    // "./runtest.exe -w entry-static.exe strtold",
    // "./runtest.exe -w entry-static.exe swprintf",
    // "./runtest.exe -w entry-static.exe tgmath",
    // "./runtest.exe -w entry-static.exe time",
    // "./runtest.exe -w entry-static.exe tls_align",
    // "./runtest.exe -w entry-static.exe udiv",
    // "./runtest.exe -w entry-static.exe ungetc",
    // "./runtest.exe -w entry-static.exe utime",
    // "./runtest.exe -w entry-static.exe wcsstr",
    // "./runtest.exe -w entry-static.exe wcstol",
    // "./runtest.exe -w entry-static.exe pleval",
    // "./runtest.exe -w entry-static.exe daemon_failure",
    // "./runtest.exe -w entry-static.exe dn_expand_empty",
    // "./runtest.exe -w entry-static.exe dn_expand_ptr_0",
    // "./runtest.exe -w entry-static.exe fflush_exit",
    // "./runtest.exe -w entry-static.exe fgets_eof",
    // "./runtest.exe -w entry-static.exe fgetwc_buffering",
    // "./runtest.exe -w entry-static.exe fpclassify_invalid_ld80",
    // "./runtest.exe -w entry-static.exe ftello_unflushed_append",
    // "./runtest.exe -w entry-static.exe getpwnam_r_crash",
    // "./runtest.exe -w entry-static.exe getpwnam_r_errno",
    // "./runtest.exe -w entry-static.exe iconv_roundtrips",
    // "./runtest.exe -w entry-static.exe inet_ntop_v4mapped",
    // "./runtest.exe -w entry-static.exe inet_pton_empty_last_field",
    // "./runtest.exe -w entry-static.exe iswspace_null",
    // "./runtest.exe -w entry-static.exe lrand48_signextend",
    // "./runtest.exe -w entry-static.exe lseek_large",
    // "./runtest.exe -w entry-static.exe malloc_0",
    // "./runtest.exe -w entry-static.exe mbsrtowcs_overflow",
    // "./runtest.exe -w entry-static.exe memmem_oob_read",
    // "./runtest.exe -w entry-static.exe memmem_oob",
    // "./runtest.exe -w entry-static.exe mkdtemp_failure",
    // "./runtest.exe -w entry-static.exe mkstemp_failure",
    // "./runtest.exe -w entry-static.exe printf_1e9_oob",
    // "./runtest.exe -w entry-static.exe printf_fmt_g_round",
    // "./runtest.exe -w entry-static.exe printf_fmt_g_zeros",
    // "./runtest.exe -w entry-static.exe printf_fmt_n",
    // "./runtest.exe -w entry-static.exe pthread_robust_detach",
    // "./runtest.exe -w entry-static.exe pthread_cancel_sem_wait",
    // "./runtest.exe -w entry-static.exe pthread_cond_smasher",
    // "./runtest.exe -w entry-static.exe pthread_condattr_setclock",
    // "./runtest.exe -w entry-static.exe pthread_exit_cancel",
    // "./runtest.exe -w entry-static.exe pthread_once_deadlock",
    // "./runtest.exe -w entry-static.exe pthread_rwlock_ebusy",
    // "./runtest.exe -w entry-static.exe putenv_doublefree",
    // "./runtest.exe -w entry-static.exe regex_backref_0",
    // "./runtest.exe -w entry-static.exe regex_bracket_icase",
    // "./runtest.exe -w entry-static.exe regex_ere_backref",
    // "./runtest.exe -w entry-static.exe regex_escaped_high_byte",
    // "./runtest.exe -w entry-static.exe regex_negated_range",
    // "./runtest.exe -w entry-static.exe regexec_nosub",
    // "./runtest.exe -w entry-static.exe rewind_clear_error",
    // "./runtest.exe -w entry-static.exe rlimit_open_files",
    // "./runtest.exe -w entry-static.exe scanf_bytes_consumed",
    // "./runtest.exe -w entry-static.exe scanf_match_literal_eof",
    // "./runtest.exe -w entry-static.exe scanf_nullbyte_char",
    // "./runtest.exe -w entry-static.exe setvbuf_unget",
    // "./runtest.exe -w entry-static.exe sigprocmask_internal",
    // "./runtest.exe -w entry-static.exe sscanf_eof",
    // "./runtest.exe -w entry-static.exe stavfs",
    // "./runtest.exe -w entry-static.exe strverscmp",
    // "./runtest.exe -w entry-static.exe syscall_sign_extend",
    // "./runtest.exe -w entry-static.exe uselocale_0",
    // "./runtest.exe -w entry-static.exe wcsncpy_read_overflow",
    // "./runtest.exe -w entry-static.exe wcsstr_false_negative",
];

pub const FORMAT_LIBC_DYNAMIC: &[&str] = &[
    "./runtest.exe -w entry-dynamic.exe argv",
    "./runtest.exe -w entry-dynamic.exe basename",
    "./runtest.exe -w entry-dynamic.exe clocale_mbfuncs",
    "./runtest.exe -w entry-dynamic.exe clock_gettime",
    "./runtest.exe -w entry-dynamic.exe crypt",
    "./runtest.exe -w entry-dynamic.exe dirname",
    "./runtest.exe -w entry-dynamic.exe dlopen",
    "./runtest.exe -w entry-dynamic.exe env",
    "./runtest.exe -w entry-dynamic.exe fdopen",
    "./runtest.exe -w entry-dynamic.exe fnmatch",
    "./runtest.exe -w entry-dynamic.exe fscanf",
    "./runtest.exe -w entry-dynamic.exe fwscanf",
    "./runtest.exe -w entry-dynamic.exe iconv_open",
    "./runtest.exe -w entry-dynamic.exe inet_pton",
    "./runtest.exe -w entry-dynamic.exe mbc",
    "./runtest.exe -w entry-dynamic.exe memstream",
    "./runtest.exe -w entry-dynamic.exe pthread_cancel_points",
    "./runtest.exe -w entry-dynamic.exe pthread_cancel",
    "./runtest.exe -w entry-dynamic.exe pthread_cond",
    "./runtest.exe -w entry-dynamic.exe pthread_tsd",
    "./runtest.exe -w entry-dynamic.exe qsort",
    "./runtest.exe -w entry-dynamic.exe random",
    "./runtest.exe -w entry-dynamic.exe search_hsearch",
    "./runtest.exe -w entry-dynamic.exe search_insque",
    "./runtest.exe -w entry-dynamic.exe search_lsearch",
    "./runtest.exe -w entry-dynamic.exe search_tsearch",
    "./runtest.exe -w entry-dynamic.exe sem_init",
    "./runtest.exe -w entry-dynamic.exe setjmp",
    "./runtest.exe -w entry-dynamic.exe snprintf",
    "./runtest.exe -w entry-dynamic.exe socket",
    "./runtest.exe -w entry-dynamic.exe sscanf",
    "./runtest.exe -w entry-dynamic.exe sscanf_long",
    "./runtest.exe -w entry-dynamic.exe stat",
    "./runtest.exe -w entry-dynamic.exe strftime",
    "./runtest.exe -w entry-dynamic.exe string",
    "./runtest.exe -w entry-dynamic.exe string_memcpy",
    "./runtest.exe -w entry-dynamic.exe string_memmem",
    "./runtest.exe -w entry-dynamic.exe string_memset",
    "./runtest.exe -w entry-dynamic.exe string_strchr",
    "./runtest.exe -w entry-dynamic.exe string_strcspn",
    "./runtest.exe -w entry-dynamic.exe string_strstr",
    "./runtest.exe -w entry-dynamic.exe strptime",
    "./runtest.exe -w entry-dynamic.exe strtod",
    "./runtest.exe -w entry-dynamic.exe strtod_simple",
    "./runtest.exe -w entry-dynamic.exe strtof",
    "./runtest.exe -w entry-dynamic.exe strtol",
    "./runtest.exe -w entry-dynamic.exe strtold",
    "./runtest.exe -w entry-dynamic.exe swprintf",
    "./runtest.exe -w entry-dynamic.exe tgmath",
    "./runtest.exe -w entry-dynamic.exe time",
    "./runtest.exe -w entry-dynamic.exe tls_init",
    "./runtest.exe -w entry-dynamic.exe tls_local_exec",
    "./runtest.exe -w entry-dynamic.exe udiv",
    "./runtest.exe -w entry-dynamic.exe ungetc",
    "./runtest.exe -w entry-dynamic.exe utime",
    "./runtest.exe -w entry-dynamic.exe wcsstr",
    "./runtest.exe -w entry-dynamic.exe wcstol",
    "./runtest.exe -w entry-dynamic.exe daemon_failure",
    "./runtest.exe -w entry-dynamic.exe dn_expand_empty",
    "./runtest.exe -w entry-dynamic.exe dn_expand_ptr_0",
    "./runtest.exe -w entry-dynamic.exe fflush_exit",
    "./runtest.exe -w entry-dynamic.exe fgets_eof",
    "./runtest.exe -w entry-dynamic.exe fgetwc_buffering",
    "./runtest.exe -w entry-dynamic.exe fpclassify_invalid_ld80",
    "./runtest.exe -w entry-dynamic.exe ftello_unflushed_append",
    "./runtest.exe -w entry-dynamic.exe getpwnam_r_crash",
    "./runtest.exe -w entry-dynamic.exe getpwnam_r_errno",
    "./runtest.exe -w entry-dynamic.exe iconv_roundtrips",
    "./runtest.exe -w entry-dynamic.exe inet_ntop_v4mapped",
    "./runtest.exe -w entry-dynamic.exe inet_pton_empty_last_field",
    "./runtest.exe -w entry-dynamic.exe iswspace_null",
    "./runtest.exe -w entry-dynamic.exe lrand48_signextend",
    "./runtest.exe -w entry-dynamic.exe lseek_large",
    "./runtest.exe -w entry-dynamic.exe malloc_0",
    "./runtest.exe -w entry-dynamic.exe mbsrtowcs_overflow",
    "./runtest.exe -w entry-dynamic.exe memmem_oob_read",
    "./runtest.exe -w entry-dynamic.exe memmem_oob",
    "./runtest.exe -w entry-dynamic.exe mkdtemp_failure",
    "./runtest.exe -w entry-dynamic.exe mkstemp_failure",
    "./runtest.exe -w entry-dynamic.exe printf_1e9_oob",
    "./runtest.exe -w entry-dynamic.exe printf_fmt_g_round",
    "./runtest.exe -w entry-dynamic.exe printf_fmt_g_zeros",
    "./runtest.exe -w entry-dynamic.exe printf_fmt_n",
    "./runtest.exe -w entry-dynamic.exe pthread_robust_detach",
    "./runtest.exe -w entry-dynamic.exe pthread_cond_smasher",
    "./runtest.exe -w entry-dynamic.exe pthread_condattr_setclock",
    "./runtest.exe -w entry-dynamic.exe pthread_exit_cancel",
    "./runtest.exe -w entry-dynamic.exe pthread_once_deadlock",
    "./runtest.exe -w entry-dynamic.exe pthread_rwlock_ebusy",
    "./runtest.exe -w entry-dynamic.exe putenv_doublefree",
    "./runtest.exe -w entry-dynamic.exe regex_backref_0",
    "./runtest.exe -w entry-dynamic.exe regex_bracket_icase",
    "./runtest.exe -w entry-dynamic.exe regex_ere_backref",
    "./runtest.exe -w entry-dynamic.exe regex_escaped_high_byte",
    "./runtest.exe -w entry-dynamic.exe regex_negated_range",
    "./runtest.exe -w entry-dynamic.exe regexec_nosub",
    "./runtest.exe -w entry-dynamic.exe rewind_clear_error",
    "./runtest.exe -w entry-dynamic.exe rlimit_open_files",
    "./runtest.exe -w entry-dynamic.exe scanf_bytes_consumed",
    "./runtest.exe -w entry-dynamic.exe scanf_match_literal_eof",
    "./runtest.exe -w entry-dynamic.exe scanf_nullbyte_char",
    "./runtest.exe -w entry-dynamic.exe setvbuf_unget",
    "./runtest.exe -w entry-dynamic.exe sigprocmask_internal",
    "./runtest.exe -w entry-dynamic.exe sscanf_eof",
    "./runtest.exe -w entry-dynamic.exe stavfs",
    "./runtest.exe -w entry-dynamic.exe strverscmp",
    "./runtest.exe -w entry-dynamic.exe syscall_sign_extend",
    "./runtest.exe -w entry-dynamic.exe tls_get_new_dtv",
    "./runtest.exe -w entry-dynamic.exe uselocale_0",
    "./runtest.exe -w entry-dynamic.exe wcsncpy_read_overflow",
    "./runtest.exe -w entry-dynamic.exe wcsstr_false_negative",
];
