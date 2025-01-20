#include <stdio.h>
#include <stdlib.h>


struct node {
    int value;
    struct node *next;
};


void displayLL(struct node *p){
    if(p){
        do{
            printf("value: %d ", p->value);
            printf("| next: %p\n", p->next);
            p=p->next;
        }while(p);
    }
    else{
        printf("No data");
    }
};

int main() {
    struct node *node1 = malloc(sizeof(struct node));
    struct node *node2 = malloc(sizeof(struct node));
    struct node *node3 = malloc(sizeof(struct node));
    struct node *node4 = malloc(sizeof(struct node));
    
    node1->value = 10;
    node2->value = 20;
    node3->value = 30;
    node4->value = 40;
    
    node1->next = node2;
    node2->next = node3;
    node3->next = node4;
    node4->next = NULL;
    
    
    displayLL(node1);
    return 0;
}