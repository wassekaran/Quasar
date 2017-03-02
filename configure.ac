#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.69])
AC_INIT([FULL-PACKAGE-NAME], [VERSION], [BUG-REPORT-ADDRESS])

# Checks for programs.
AC_CHECK_PROG([RUSTC], [rustc], [rustc], [no])
AC_CHECK_PROG([NASM], [nasm], [nasm], [no])
AC_CHECK_TOOL([LD], [ld], [ld], [no])
test x"$RUSTC" == x"no" && AC_MSG_ERROR([Required program 'rustc' not found.])
test x"$NASM" == x"no" && AC_MSG_ERROR([Required program 'nasm' not found.])
test x"$LD" == x"no" && AC_MSG_ERROR([Required program 'ld' not found.])

AC_CONFIG_FILES([Makefile])
AC_OUTPUT
