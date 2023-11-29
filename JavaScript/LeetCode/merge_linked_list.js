// https://leetcode.com/problems/merge-two-sorted-lists/
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */

/**
 * @param {ListNode} list1
 * @param {ListNode} list2
 * @return {ListNode}
 */
var mergeTwoLists = function(list1, list2) {
    if (!list1) return list2;
    if (!list2) return list1;

    const currentNode = new ListNode();
    const head = currentNode;
    while(list1 && list2){
        if(list1.val <= list2.val){
            currentNode.next = new ListNode(list1.val);
            list1 = list1.next;
        }
        else {
            currentNode.next = new ListNode(list2.val);
            list2 = list2.next;
        }
        currentNode = currentNode.next;
    }
    while(list1){
        currentNode.next = new ListNode(list1.val);
        list1 = list1.next;
        currentNode = currentNode.next;
    }
    while(list2){
        currentNode.next = new ListNode(list2.val);
        list2 = list2.next
        currentNode = currentNode.next
    }
    return head.next;
};