class Solution {
public:
    int islandPerimeter(vector<vector<int>>& grid) {
        vector<vector<bool>>visited(
            grid.size(),
            vector<bool>(grid[0].size(),false)
        );
        int r = grid.size();
        int c = grid[0].size();
        for(int i = 0 ; i < r ; i++) {
            for(int j = 0 ; j < c ; j++) {
                if (grid[i][j] == 1) {
                   return dfs(grid,visited,j,i);
                }
            }
        }
    }

    int dfs(vector<vector<int>>& grid, vector<vector<bool>>& v, int x, int y) {
        int r = grid.size()-1;
        int c = grid[0].size()-1;
        if (x < 0 || x > c || y < 0 || y > r) {
            return 1;
        } else if (v[y][x]) {
            return 0;
        } else if (grid[y][x] == 0) {
            return 1;
        }else {
            v[y][x] = true;
            return dfs(grid,v,x+1,y) + dfs(grid,v,x-1,y) + 
            dfs(grid,v,x,y+1) + dfs(grid,v,x,y-1);
        }
    }
};