(function ()
{

function range(start, step, end)
{
    var array = [];
    for (var n = 0; n < end - start; n += step)
    {
        array[n] = n + start;
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

})();
