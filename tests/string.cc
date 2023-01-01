#include <gmock/gmock.h>
#include <gtest/gtest.h>

extern "C" {
  void *gi_memccpy(void *__restrict, const void *__restrict, int, size_t);
  void *gi_memchr(const void *, int, size_t);
  int gi_memcmp(const void *, const void *, size_t);
  void *gi_memcpy(void *__restrict, const void *__restrict, size_t);
  void *gi_memmove(void *, const void *, size_t);
  void *gi_memrchr(const void *, int, size_t);
  void *gi_memset(void *, int, size_t);
  wchar_t *gi_wmemchr(const wchar_t *, wchar_t, size_t);
  int gi_wmemcmp(const wchar_t *, const wchar_t *, size_t);
  wchar_t *gi_wmemcpy(wchar_t *__restrict, const wchar_t *__restrict, size_t);
  wchar_t *gi_wmemmove(wchar_t *, const wchar_t *, size_t);
  wchar_t *gi_wmemset(wchar_t *, wchar_t, size_t);
  char *gi_stpcpy(char *__restrict, const char *__restrict);
  char *gi_stpncpy(char *__restrict, const char *__restrict, size_t);
  char *gi_strchr(const char *, int);
  char *gi_strrchr(const char *, int);
  char *gi_strcat(char *s, const char *append);
  char *gi_strncat(char *dst, const char *src, size_t n);
  char *gi_strcpy(char *to, const char *from);
  char *gi_strncpy(char *dst, const char *src, size_t n);
  int gi_strncmp(const char *s1, const char *s2, size_t n);
  int gi_strcmp(const char *, const char *);
}

TEST(memccpy, null) {
  ASSERT_EQ(NULL, gi_memccpy((char *)456, (char *)789, 'A', 0));
}

TEST(memccpy, example) {
  const char buf1[13] = "Test\0string!";
  char buf2[] = "AAAAAAAAA";
  ASSERT_EQ(&buf2[8], gi_memccpy(buf2, buf1, 'r', 9999));
  ASSERT_THAT(buf2, testing::ElementsAreArray("Test\0strA"));
}

TEST(STRING_TEST, memccpy_smoke) {
  char dst[32];
  memset(dst, 0, sizeof(dst));
  char* p = static_cast<char*>(gi_memccpy(dst, "hello world", ' ', 32));
  ASSERT_STREQ("hello ", dst);
  ASSERT_EQ(ptrdiff_t(6), p - dst);
  memset(dst, 0, sizeof(dst));
  ASSERT_EQ(nullptr, gi_memccpy(dst, "hello world", ' ', 4));
  ASSERT_STREQ("hell", dst);
}

TEST(memchr, null) {
  ASSERT_EQ(NULL, gi_memchr((char *)nullptr, 'A', 0));
}

TEST(memchr, match) {
  char buf[] = "Foo bar baz";
  ASSERT_EQ(buf + 5, gi_memchr(buf, 'a', sizeof(buf)));
}

TEST(memchr, nomatch) {
  char buf[] = "Foo bar baz";
  ASSERT_EQ(NULL, gi_memchr(buf, 'x', sizeof(buf)));
}

TEST(memcmp, null) {
  ASSERT_EQ(0, gi_memcmp(NULL, NULL, 0));
}

TEST(memcmp, example) {
  const char buf1[] = "Hello";
  const char buf2[] = "Helxo";
  ASSERT_EQ(0, gi_memcmp(buf1, buf1, sizeof(buf1)));
  ASSERT_GT(0, gi_memcmp(buf1, buf2, sizeof(buf1)));
  ASSERT_LT(0, gi_memcmp(buf2, buf1, sizeof(buf1)));
}

TEST(memcpy, null) {
  ASSERT_EQ((char *)42, gi_memcpy((char *)42, (char *)123, 0));
}

TEST(memcpy, example) {
  const char buf1[8] = "Foo\0Bar";
  char buf2[8];
  ASSERT_EQ(buf2, gi_memcpy(buf2, buf1, sizeof(buf1)));
  ASSERT_THAT(buf2, testing::ElementsAreArray(buf1));
}

TEST(memmove, null) {
  ASSERT_EQ((char *)42, gi_memmove((char *)42, (char *)34, 0));
}

TEST(memmove, example1) {
  char buf[] = "abcdefghijkl";
  ASSERT_EQ(buf, gi_memmove(buf, buf + 4, 8));
  ASSERT_STREQ("efghijklijkl", buf);
}

TEST(memmove, example2) {
  char buf[] = "abcdefghijkl";
  ASSERT_EQ(buf + 4, gi_memmove(buf + 4, buf, 8));
  ASSERT_STREQ("abcdabcdefgh", buf);
}

TEST(memrchr, null) {
  ASSERT_EQ(NULL, gi_memrchr(NULL, 'A', 0));
}

TEST(memrchr, match) {
  char buf[] = "Foo bar baz";
  ASSERT_EQ(buf + 9, gi_memrchr(buf, 'a', sizeof(buf)));
}

TEST(memrchr, nomatch) {
  char buf[] = "Foo bar baz";
  ASSERT_EQ(NULL, gi_memrchr(buf, 'x', sizeof(buf)));
}

TEST(memset, null) {
  ASSERT_EQ((char *)5, gi_memset((char *)5, 'A', 0));
}

TEST(memset, example_small) {
  char buf[11];
  ASSERT_EQ(buf, gi_memset(buf, '!', 10));
  buf[10] = '\0';
  ASSERT_STREQ("!!!!!!!!!!", buf);
}

TEST(memset, example_large) {
  char buf[101];
  ASSERT_EQ(buf, gi_memset(buf, '!', 100));
  buf[100] = '\0';
  ASSERT_THAT(buf,
              testing::ElementsAreArray("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
                                        "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
                                        "!!!!!!!!!!!!!!!!!!!!!!!!"));
}

TEST(wmemchr, null) {
  ASSERT_EQ(NULL, gi_wmemchr((wchar_t *)NULL, L'A', 0));
}

TEST(wmemchr, match) {
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(buf + 5, gi_wmemchr(buf, L'a', std::size(buf)));
}

TEST(wmemchr, nomatch) {
  wchar_t buf[] = L"Foo bar baz";
  ASSERT_EQ(NULL, gi_wmemchr(buf, L'x', std::size(buf)));
}

TEST(wmemcmp, null) {
  ASSERT_EQ(0, gi_wmemcmp(NULL, NULL, 0));
}

TEST(wmemcmp, example) {
  const wchar_t buf1[] = L"Hello";
  const wchar_t buf2[] = L"Helxo";
  ASSERT_EQ(0, gi_wmemcmp(buf1, buf1, std::size(buf1)));
  ASSERT_GT(0, gi_wmemcmp(buf1, buf2, std::size(buf1)));
  ASSERT_LT(0, gi_wmemcmp(buf2, buf1, std::size(buf1)));
}

TEST(wmemcpy, null) {
  ASSERT_EQ((wchar_t *)42, gi_wmemcpy((wchar_t *)42, (wchar_t *)123, 0));
}

TEST(wmemcpy, example) {
  const wchar_t buf1[8] = L"Foo\0Bar";
  wchar_t buf2[8];
  ASSERT_EQ(buf2, gi_wmemcpy(buf2, buf1, std::size(buf1)));
  ASSERT_THAT(buf2, testing::ElementsAreArray(buf1));
}

TEST(wmemmove, null) {
  ASSERT_EQ((wchar_t *)42, gi_wmemmove((wchar_t *)42, (wchar_t *)34, 0));
}

TEST(wmemmove, example1) {
  wchar_t buf[] = L"abcdefghijkl";
  ASSERT_EQ(buf, gi_wmemmove(buf, buf + 4, 8));
  ASSERT_STREQ(L"efghijklijkl", buf);
}

TEST(wmemmove, example2) {
  wchar_t buf[] = L"abcdefghijkl";
  ASSERT_EQ(buf + 4, gi_wmemmove(buf + 4, buf, 8));
  ASSERT_STREQ(L"abcdabcdefgh", buf);
}

TEST(wmemset, null) {
  ASSERT_EQ((wchar_t *)5, gi_wmemset((wchar_t *)5, L'A', 0));
}

TEST(wmemset, example) {
  wchar_t buf[11];
  ASSERT_EQ(buf, gi_wmemset(buf, L'!', 10));
  buf[10] = L'\0';
  ASSERT_STREQ(L"!!!!!!!!!!", buf);
}

TEST(stpcpy, example) {
  char buf[] = "AAAAAAAAAA";
  ASSERT_EQ(buf, gi_stpcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf + 5, gi_stpcpy(buf, "Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, gi_stpcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0ello\0AAAA"));
  ASSERT_EQ(buf + 9, gi_stpcpy(buf, "Example!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Example!!\0"));
}

TEST(stpncpy, null) {
  ASSERT_EQ((char *)12, gi_stpncpy((char *)12, (char *)500, 0));
}

TEST(stpncpy, example1) {
  char buf[] = "AAAAAAAAAAAA";
  ASSERT_EQ(buf + 5, gi_stpncpy(buf, "Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0\0\0\0\0\0\0"));
}

TEST(stpncpy, example2) {
  char buf[] = "AAAAAAAAAAAA";
  ASSERT_EQ(buf + 12, gi_stpncpy(buf, "This is a very long string", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray("This is a ve"));
}

TEST(strchr, examples) {
  const char *str = "Hello, world";
  ASSERT_EQ(NULL, gi_strchr(str, 'A'));
  ASSERT_EQ(str + 4, gi_strchr(str, 'o'));
  ASSERT_EQ(str + 12, gi_strchr(str, '\0'));
}

TEST(strrchr, examples) {
  const char *str = "Hello, world";
  ASSERT_EQ(NULL, gi_strrchr(str, 'A'));
  ASSERT_EQ(str + 8, gi_strrchr(str, 'o'));
  ASSERT_EQ(str + 12, gi_strrchr(str, '\0'));
}

TEST(strncat, example) {
  char buf[] = "\0AAAAAAAAA";
  ASSERT_EQ(buf, gi_strncat(buf, "", 0));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf, gi_strncat(buf, "Hello", 99999));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, gi_strncat(buf, "", 1));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, gi_strncat(buf, "!!!!!!!!!!!!", 3));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello!!!\0A"));
}

TEST(strcat, example) {
  char buf[] = "\0AAAAAAAAA";
  ASSERT_EQ(buf, gi_strcat(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf, gi_strcat(buf, "Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, gi_strcat(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, gi_strcat(buf, "!!!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello!!!!\0"));
}

TEST(strncmp, null) {
  ASSERT_EQ(0, strncmp(NULL, NULL, 0));
}

TEST(strncmp, examples) {
  ASSERT_EQ(0, gi_strncmp("", "", 100));
  ASSERT_EQ(0, gi_strncmp("Hello", "Hello", 100));

  ASSERT_EQ(0, gi_strncmp("Hello", "Hello, world", 5));
  ASSERT_GT(0, gi_strncmp("Hello", "Hello, world", 6));
  ASSERT_LT(0, gi_strncmp("Hello, world", "Hello", 100));

  ASSERT_EQ(0, gi_strncmp("Hello!", "Hello.", 5));
  ASSERT_GT(0, gi_strncmp("Hello!", "Hello.", 6));
  ASSERT_LT(0, gi_strncmp("Hello.", "Hello!", 100));
}

TEST(strcmp, examples) {
  ASSERT_EQ(0, gi_strcmp("", ""));
  ASSERT_EQ(0, gi_strcmp("Hello", "Hello"));

  ASSERT_GT(0, gi_strcmp("Hello", "Hello, world"));
  ASSERT_LT(0, gi_strcmp("Hello, world", "Hello"));

  ASSERT_GT(0, gi_strcmp("Hello!", "Hello."));
  ASSERT_LT(0, gi_strcmp("Hello.", "Hello!"));
}

TEST(strncpy, null) {
  ASSERT_EQ((char *)12, gi_strncpy((char *)12, (char *)500, 0));
}

TEST(strncpy, example1) {
  char buf[] = "AAAAAAAAAAAA";
  ASSERT_EQ(buf, gi_strncpy(buf, "Hello", 12));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0\0\0\0\0\0\0"));
}

TEST(strncpy, example2) {
  char buf[13];
  ASSERT_EQ(buf, gi_strncpy(buf, "This is a very long string", 12));
  buf[12] = '\0';
  ASSERT_THAT(buf, testing::ElementsAreArray("This is a ve"));
}

TEST(strcpy, example) {
  char buf[] = "AAAAAAAAAA";
  ASSERT_EQ(buf, gi_strcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0AAAAAAAAA"));
  ASSERT_EQ(buf, gi_strcpy(buf, "Hello"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Hello\0AAAA"));
  ASSERT_EQ(buf, gi_strcpy(buf, ""));
  ASSERT_THAT(buf, testing::ElementsAreArray("\0ello\0AAAA"));
  ASSERT_EQ(buf, gi_strcpy(buf, "Example!!"));
  ASSERT_THAT(buf, testing::ElementsAreArray("Example!!\0"));
}
