(function ()
{

function range(start, step, end)
{
    var array = [];
    for (start; start < end; start += step)
    {
        array.push(start);
    }
    return array;
}

function print(x)
{
    console.log(x);
}

var isDivisible = function (n)
{
    return function (x)
    {
        return x % n === 0;
    }
}

var isFizz = isDivisible(3);
var isBuzz = isDivisible(5);
var isFizzBuzz = function (x) { return isFizz(x) && isBuzz(x) };

var fizzBuzz = function (n) {
    if (isFizzBuzz(n))
    {
        return "FizzBuzz";
    }
    else if (isFizz(n))
    {
        return "Fizz";
    }
    else if (isBuzz(n))
    {
        return "Buzz";
    }
    else
    {
        return String(n);
    }
}

range(1,1,100).map(function (n) {return fizzBuzz(n)}).forEach(print);

})();
