def is_palindrome(n: int) -> bool:
    s = str(n)
    l = len(s)

    left, right = (s[:l//2], s[l//2:]) if l % 2 == 0 else (s[:l//2], s[l//2+1:])

    return left == right[::-1]


def main():
    assert is_palindrome(121)
    assert is_palindrome(1331)
    assert not is_palindrome(-121)
    assert not is_palindrome(10)


if __name__ == "__main__":
    main()
