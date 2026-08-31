/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

class Solution {
public:
    int goodNodesHelper (TreeNode* root, int max) {
        int addition = 0;
        if (root == nullptr) {
            return 0;
        } else {
            if (root->val >= max) {
                addition++;
                max = root->val;
            }
            return addition + goodNodesHelper(root->left, max) + goodNodesHelper(root->right, max);
        }
    }
    int goodNodes(TreeNode* root) {
        return goodNodesHelper(root, root->val);
    }
};
