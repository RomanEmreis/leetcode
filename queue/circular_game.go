/*
  There are n friends that are playing a game. The friends are sitting in a circle and are numbered from 1 to n in clockwise order. More formally, 
  moving clockwise from the ith friend brings you to the (i+1)th friend for 1 <= i < n, and moving clockwise from the nth friend brings you to the 1st friend.

  The rules of the game are as follows:

  Start at the 1st friend.
  Count the next k friends in the clockwise direction including the friend you started at. The counting wraps around the circle and may count some friends more than once.
  The last friend you counted leaves the circle and loses the game.
  If there is still more than one friend in the circle, go back to step 2 starting from the friend immediately clockwise of the friend who just lost and repeat.
  Else, the last friend in the circle wins the game.
  Given the number of friends, n, and an integer k, return the winner of the game.

  Example:
    Input: n = 5, k = 2
    Output: 3
    Explanation: Here are the steps of the game:
    1) Start at friend 1.
    2) Count 2 friends clockwise, which are friends 1 and 2.
    3) Friend 2 leaves the circle. Next start is friend 3.
    4) Count 2 friends clockwise, which are friends 3 and 4.
    5) Friend 4 leaves the circle. Next start is friend 5.
    6) Count 2 friends clockwise, which are friends 5 and 1.
    7) Friend 1 leaves the circle. Next start is friend 3.
    8) Count 2 friends clockwise, which are friends 3 and 5.
    9) Friend 5 leaves the circle. Only friend 3 is left, so they are the winner.
*/
func findTheWinner(n int, k int) int {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        if k % 2 == 0 {
            return 1;
        }
        return 2;
    }

    result := solve(n - 1, k);
    if result > n {
        return result % n;
    }
    return result;
}

func solve(n, k int) int {
    if n == 2 {
        if k % 2 == 0 {
            return 1 + k;
        }
        return k + 2;
    }

    pos := solve(n - 1, k);
    if pos > n {
        pos %= n;
    }
    if pos == 0 {
        pos = n;
    }

    return pos + k;
}
