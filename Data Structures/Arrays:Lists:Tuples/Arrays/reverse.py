# Reverse A Array
def reverse(data):
    return data[::-1]

if __name__ == '__main__':

    arr = list(map(int, input().rstrip().split()))

    print(reverse(arr))