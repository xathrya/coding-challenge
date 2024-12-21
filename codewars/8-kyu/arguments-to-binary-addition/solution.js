// https://www.codewars.com/kata/57642a90dee2da8dd3000161

// solution 1: 837ms
function arr2bin(arr) {
    return arr
        .reduce((total, value) => (typeof value === 'number' ? total + value : total), 0)
        .toString(2);
}
