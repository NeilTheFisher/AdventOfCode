def find_loop(grid, start):
    def get_neighbors(x, y):
        neighbors = []
        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if 0 <= nx < rows and 0 <= ny < cols and grid[nx][ny] != '.' and (nx, ny) not in loop:
                neighbors.append((nx, ny))
        return neighbors

    rows, cols = len(grid), len(grid[0])
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]

    loop = set()
    stack = [start]

    while stack:
        current = stack.pop()
        loop.add(current)
        neighbors = get_neighbors(*current)
        stack.extend(neighbors)

    return loop


def calculate_distances(grid, start, loop):
    distances = [[float('inf')] * len(grid[0]) for _ in range(len(grid))]
    distances[start[0]][start[1]] = 0
    queue = [start]

    while queue:
        x, y = queue.pop(0)
        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nx, ny = x + dx, y + dy
            if 0 <= nx < len(grid) and 0 <= ny < len(grid[0]) and grid[nx][ny] != '.' and distances[nx][ny] == float('inf'):
                distances[nx][ny] = distances[x][y] + 1
                queue.append((nx, ny))

    max_distance = 0
    for x, y in loop:
        max_distance = max(max_distance, distances[x][y])

    return max_distance

def read_input_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.read().splitlines()
    return lines

def main():
    # grid = [
    #     "..F7.",
    #     ".FJ|.",
    #     "SJ.L7",
    #     "|F--J",
    #     "LJ..."
    # ]

    grid = read_input_file("input.txt")

    start = None
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if grid[i][j] == 'S':
                start = (i, j)

    loop = find_loop(grid, start)
    max_distance = calculate_distances(grid, start, loop)

    print("The maximum distance along the loop is:", max_distance)


if __name__ == "__main__":
    main()
