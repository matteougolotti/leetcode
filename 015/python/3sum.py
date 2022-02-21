from typing import List


def three_sum(nums: List[int]) -> List[List[int]]:
    indexes = {v: i for i, v in enumerate(nums)}
    triplets = set()
    visited_i = set()
    for i_i, i in enumerate(nums):
        print(i)
        if i in visited_i:
            continue
        else:
            visited_i.add(i)
        for j_i, j in enumerate(nums):
            k = -(i + j)
            if k in indexes:
                k_i = indexes[k]
                if i_i != j_i and i_i != k_i and j_i != k_i:
                    triplets.add(tuple(sorted((i, j, k))))

    return list(sorted([list(t) for t in triplets]))



def main():
    assert [[-1,-1,2],[-1,0,1]] == three_sum(
        [-1,0,1,2,-1,-4]
    )


if __name__ == "__main__":
    main()
