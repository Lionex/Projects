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

})();
