#include <stdio.h>
#include <stdlib.h>

typedef struct node {
   int value;
   struct node *next;
}Node;


Node* create_node(int value){
    Node* node = malloc(sizeof(Node));
    node->value = value;
    node->next = NULL;

    return node;
}


void displayLL(Node *p){
    if(p){
        do{
            printf("value: %d ", p->value);
            printf("| memory adress: %p\n", p->next);
            p=p->next;
        }while(p);
    }
    else{
        printf("No data");
    }
};

Node* insertPrev(Node *head, int value){
    Node* newNode = malloc(sizeof(Node));
    newNode->value = value;
    newNode->next = head;

    return newNode;
};


Node* insertPos(Node *head, int value){
    Node* newNode = create_node(value);
     while (head->next != NULL) {
        head = head->next;
    }
    head->next = newNode;
    newNode->next = NULL;

    return head;
};

int main() {
    Node* head = create_node(1);
    head->next = create_node(2);
    head->next->next = create_node(3);
    head->next->next->next = create_node(4);

    printf("\nPrevious list\n");
    displayLL(head);

    insertPos(head, 5);
    printf("\nInsert 5 in the end\n");
    displayLL(head);
    
    head = insertPrev(head, 0);
    printf("\nInsert 0 in the start\n");
    displayLL(head);


    return 0;
};