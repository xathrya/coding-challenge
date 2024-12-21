// https://www.codewars.com/kata/541629460b198da04e000bb9

// 649ms
function last(list) {
    var last = arguments[arguments.length - 1];
    return last[last.length - 1] || last;
}