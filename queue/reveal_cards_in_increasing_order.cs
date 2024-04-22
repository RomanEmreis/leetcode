public class Solution {
    public int[] DeckRevealedIncreasing(int[] deck) {
        if (deck.Length == 1) return deck;

        Array.Sort(deck);

        int[] result = new int[deck.Length];
        Queue<int> indexes = [];

        for (int i = 0; i < deck.Length; ++i) indexes.Enqueue(i);

        foreach (int card in deck) {
            result[indexes.Dequeue()] = card;
            if (indexes.Any()) indexes.Enqueue(indexes.Dequeue());
        }

        return result;
    }
}
