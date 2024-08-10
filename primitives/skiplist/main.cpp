#include <random>
#include <vector>

using namespace std;

template <typename K, typename V> struct Node {
  K key;
  vector<V> values;
  Node *previous, *next, *up, *down;

  // Use for creating a head node.
  Node() : previous(nullptr), next(nullptr), up(nullptr), down(nullptr) {}

  // Use for creating a node which isn't a head / tail.
  Node(K key)
      : previous(nullptr), next(nullptr), up(nullptr), down(nullptr), key(key) {
  }
};

template <typename K, typename V> class SkipList {
private:
  // Each level's head.
  vector<Node<K, V>> heads;

  // Number of nodes (including the replicated ones).
  int size;

  // Tools required for random number generation.
  random_device randomDevice;
  uniform_real_distribution<double> randomDeviceDistribution =
      uniform_real_distribution<double>(0.0, 1.0);

public:
  void insert(K key, V value) {
    this->size++;

    // When there are no nodes in the SkipList.
    if (this->heads.empty()) {
      auto *head = new Node<K, V>();
      heads.push_back(head);

      auto *node = new Node<K, V>(key);
      node->values.push_back(value);

      head->next = node;
      node->previous = head;

      // Probabilistically replicate the node to higher levels.
      while (this->randomDeviceDistribution(this->randomDevice) > 0.5) {
        auto currentLevelHead = new Node<K, V>();
        head->up = currentLevelHead;
        currentLevelHead->down = head;

        this->heads.push_back(currentLevelHead);

        auto *replicatedNode = new Node<K, V>(key);
        node->up = replicatedNode;
        replicatedNode->down = node;

        currentLevelHead->next = replicatedNode;
        replicatedNode->previous = currentLevelHead;

        head = currentLevelHead;
        node = replicatedNode;
      }

      return;
    }

    // Find the correct position (in level 0), where the node corresponding to
    // the given key either already exists or can be inserted.

    // Suppose we're at level (x + 1) at node p. From node p, before jumping to
    // node q (at level x), which is right beneath p, push a reference to p
    // in this node storage.
    // If you create a new node at the ground level, this node storage will come
    // handy when you replicate that node to higher levels. If you need to
    // replicate the node to level (x + 1), the replica will be placed right
    // after p.
    vector<Node<K, V> *> nodeStorage;

    Node<K, V> *currentNode = this->heads.back();

    // Starting from the upper left corner.
    while (currentNode->down) {
      while (currentNode->next && currentNode->next->key < key)
        currentNode = currentNode->next;

      nodeStorage.push_back(currentNode);

      currentNode = currentNode->down;
    }

    // Reached the ground level.

    while (currentNode->next && currentNode->next->key < key)
      currentNode = currentNode->next;

    // CASE : A node corresponding to the key already exists.
    if (currentNode->next && currentNode->next->key == key) {
      currentNode = currentNode->next;
      currentNode->values.push_back(value);
      return;
    }

    // CASE : We need to create a new node corresponding to the key and
    // replicate the node probabilistically to higher levels.

    auto *newNode = new Node<K, V>(key);

    newNode->next = currentNode->next;
    if (currentNode->next != nullptr)
      currentNode->next->previous = newNode;

    currentNode->next = newNode;
    newNode->previous = currentNode;

    while (this->randomDeviceDistribution(this->randomDevice) > 0.5) {
      auto *replicatedNode = new Node<K, V>(key);

      newNode->up = replicatedNode;
      replicatedNode->down = newNode;

      newNode = replicatedNode;

      // CASE : Need to replicate the node to the upper level which already
      // exists.
      if (!nodeStorage.empty()) {
        Node<K, V> *previousNode = nodeStorage.back();

        replicatedNode->next = previousNode->next;
        if (previousNode->next != nullptr)
          previousNode->next->previous = replicatedNode;

        previousNode->next = replicatedNode;
        replicatedNode->previous = previousNode;

        nodeStorage.pop_back();

        continue;
      }

      // CASE : Need to replicate the node to the upper level which needs to be
      // created.

      auto *newHead = new Node<K, V>();

      Node<K, V> *previousLevelHead = this->heads().back();
      previousLevelHead->up = newHead;
      newHead->down = previousLevelHead;

      this->heads.push_back(newHead);

      newHead->next = replicatedNode;
      replicatedNode->previous = newHead;
    }
  }
};

int main() { return 0; }
