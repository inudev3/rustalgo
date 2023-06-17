impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0;n as usize];
        let mut ans = 0;
        backtrack(&requests,&mut cnt,n,0,0,&mut ans);
        ans
    }
}
fn backtrack(req:&Vec<Vec<i32>>, cnt:&mut Vec<i32>, n:i32, idx:usize, count:i32,  ans:&mut i32){

    if idx==req.len() {
        for i in 0..cnt.len(){
            if cnt[i]!=0{
                return
            }
        }
        *ans = std::cmp::max(*ans,count);
        return
    }

    cnt[req[idx][0] as usize]-=1;
    cnt[req[idx][1] as usize]+=1;
    backtrack(req,cnt,n,idx+1,count+1,ans);
    cnt[req[idx][0] as usize]+=1;
    cnt[req[idx][1] as usize]-=1;
    backtrack(req,cnt,n,idx+1,count,ans);
}