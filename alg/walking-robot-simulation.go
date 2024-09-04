/*
  A robot on an infinite XY-plane starts at point (0, 0) facing north. The robot can receive a sequence of these three possible types of commands:
  -2: Turn left 90 degrees.
  -1: Turn right 90 degrees.
  1 <= k <= 9: Move forward k units, one unit at a time.
  Some of the grid squares are obstacles. The ith obstacle is at grid point obstacles[i] = (xi, yi). If the robot runs into an obstacle, then it will instead stay in its current location and move on to the next command.
  
  Return the maximum Euclidean distance that the robot ever gets from the origin squared (i.e. if the distance is 5, return 25).
  
  Note:
  North means +Y direction.
  East means +X direction.
  South means -Y direction.
  West means -X direction.
  There can be obstacle in [0,0].
  
  Example 1:
    Input: commands = [4,-1,3], obstacles = []
    Output: 25
    Explanation: The robot starts at (0, 0):
    1. Move north 4 units to (0, 4).
    2. Turn right.
    3. Move east 3 units to (3, 4).
    The furthest point the robot ever gets from the origin is (3, 4), which squared is 32 + 42 = 25 units away.

  Example 2:
    Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
    Output: 65
    Explanation: The robot starts at (0, 0):
    1. Move north 4 units to (0, 4).
    2. Turn right.
    3. Move east 1 unit and get blocked by the obstacle at (2, 4), robot is at (1, 4).
    4. Turn left.
    5. Move north 4 units to (1, 8).
    The furthest point the robot ever gets from the origin is (1, 8), which squared is 12 + 82 = 65 units
*/
func robotSim(commands []int, obstacles [][]int) int {
  directions := [][]int{
		{0, 1},  // North
		{1, 0},  // East
		{0, -1}, // South
		{-1, 0}, // West
	}

	obstacleMap := make(map[[2]int]bool)
	for _, obstacle := range obstacles {
		obstacleMap[[2]int{obstacle[0], obstacle[1]}] = true
	}

	x, y := 0, 0
	dirIndex := 0
	maxDistanceSq := 0

	for _, cmd := range commands {
		if cmd == -2 {
			dirIndex = (dirIndex + 3) % 4 // Turn left
		} else if cmd == -1 {
			dirIndex = (dirIndex + 1) % 4 // Turn right
		} else {
			for i := 0; i < cmd; i++ {
				nextX, nextY := x+directions[dirIndex][0], y+directions[dirIndex][1]
				if obstacleMap[[2]int{nextX, nextY}] {
					break
				}
				x, y = nextX, nextY
				distanceSq := x*x + y*y
				if distanceSq > maxDistanceSq {
					maxDistanceSq = distanceSq
				}
			}
		}
	}
	
	return maxDistanceSq
}
