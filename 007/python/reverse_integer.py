def reverse(x: int) -> int:
    negative = x < 0

    s = str(x)
    if negative:
        s = s[1:]
        s += "-"

    r = s[::-1]

    n = int(r)

    result = n if n >= -2**31 and n < 2**31 else 0

    return result


def main():
    assert 321 == reverse(123)
    assert -321 == reverse(-123)
    assert 21 == reverse(12)


if __name__ == "__main__":
    main()
