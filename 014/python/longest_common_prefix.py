from typing import List


def longest_common_prefix(strs: List[str]) -> str:
    result = ""
    for chars in zip(*strs):
        x = chars[0] if len(chars) > 0 else None
        if all(c == x for c in chars):
            result += x
        else:
            break

    return result


def main():
    assert "fl" == longest_common_prefix([
        "flower",
        "flow",
        "flight",
    ])
    assert "" == longest_common_prefix([
        "dog",
        "racecar",
        "car",
    ])


if __name__ == "__main__":
    main()
