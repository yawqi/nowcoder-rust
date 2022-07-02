#include <unordered_map>
#include <iostream>
using namespace std;

class Node {
public:
    int elem;
    Node* next;
    Node(int elem, Node* next): elem(elem), next(next) {
    };
};

int main() {
    int n = 0, head = 0;
    unordered_map<int, Node*> map;
    cin >> n >> head;
    Node *head_node = new Node(head, NULL);
    Node dumb = Node(-1, head_node);
    map[head] = head_node;
    for (int i = 1; i < n; i++) {
        int num = 0, ins = 0;
        cin >> num >> ins;
        Node *prev = map[ins];
        Node *node = new Node(num, prev->next);
        prev->next = node;
        map[num] = node;
    }
    int del = 0;
    cin >> del;
    Node *cur = dumb.next;
    while (cur) {
        if (cur->elem != del) {
            cout << cur->elem << " ";
        }
        cur = cur->next;
    }
    return 0;
}