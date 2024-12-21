// https://www.codewars.com/kata/5bc555bb62a4cec849000047

// solution 1: 1197ms
function find(a,b,n){
    let res = a + '' + b;
    n = +String(n).slice(-4);
    while(res.length <= n)
        res += +res[res.length-2] + +res[res.length-1];
    return +res[n]
}