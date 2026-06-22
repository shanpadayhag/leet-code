impl Solution {
    pub fn add_two_numbers(
        first_number: Option<Box<ListNode>>,
        second_number: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result_head = Box::new(ListNode::new(0));
        let mut result_tail = &mut result_head;
        let mut carry = 0;

        let mut first_digit = first_number;
        let mut second_digit = second_number;

        while first_digit.is_some() || second_digit.is_some() || carry != 0 {
            let mut digit_sum = carry;

            if let Some(node) = first_digit.take() {
                digit_sum += node.val;
                first_digit = node.next;
            }
            if let Some(node) = second_digit.take() {
                digit_sum += node.val;
                second_digit = node.next;
            }

            carry = digit_sum / 10;
            result_tail.next = Some(Box::new(ListNode::new(digit_sum % 10)));
            result_tail = result_tail.next.as_mut().unwrap();
        }

        result_head.next
    }
}
