using System.Numerics;
using System.Runtime.CompilerServices;

[assembly: InternalsVisibleTo("test")]

namespace NumericalAnalysis;

public class PloyEval
{

    /// <summary>
    /// Evaluates a polynomial function with given coefficients and input values.
    /// </summary>
    /// <param name="c">The list of coefficients of the polynomial function.</param>
    /// <param name="x">The list of input values.</param>
    /// <param name="b">The optional list of bias values.</param>
    /// <returns>The list of evaluated values.</returns>
    /// <example>
    /// <code>
    /// double[] c = { 1, 2, 3 };
    /// double[] x = { 1, 2, 3 };
    /// double[] b = { 1, 2 };
    /// List&lt;double&gt; result = PloyEval.Eval(c, x, b);
    /// </code>
    /// </example>
    /// <remarks>
    /// <para>
    /// The polynomial is evaluated using the following formula:
    /// <code>
    /// y = c[0] + (x - b[0]) * (c[1] + (x - b[1]) * (c[2] + ...))
    /// </code>
    /// </para>
    /// <para>
    /// The bias is optional. If no bias is given, the bias is assumed to be zero.
    /// </para>
    /// </remarks>
    public static List<double> Eval(List<double> c, List<double> x, List<double>? b = null)
    {
        if (Vector.IsHardwareAccelerated)
        {
            return Eval_SIMD([.. c], [.. x], b?.ToArray());
        }
        return Eval_NoSIMD(c, x, b);
    }

    /// <summary>
    /// Evaluates a polynomial using the given coefficients and points.
    /// </summary>
    /// <param name="c">The coefficients of the polynomial.</param>
    /// <param name="x">The points at which to evaluate the polynomial.</param>
    /// <param name="b">Optional bias for the points.</param>
    /// <returns>A list of evaluated polynomial values.</returns>
    /// <example>
    /// <code>
    /// double[] c = { 1, 2, 3 };
    /// double[] x = { 1, 2, 3 };
    /// double[] b = { 1, 2 };
    /// List&lt;double&gt; result = PloyEval.Eval(c, x, b);
    /// </code>
    /// </example>
    /// <remarks>
    /// <para>
    /// The polynomial is evaluated using the following formula:
    /// <code>
    /// y = c[0] + (x - b[0]) * (c[1] + (x - b[1]) * (c[2] + ...))
    /// </code>
    /// </para>
    /// <para>
    /// The bias is optional. If no bias is given, the bias is assumed to be zero.
    /// </para>
    /// </remarks>
    public static List<double> Eval(double[] c, double[] x, double[]? b = null)
    {
        if (Vector.IsHardwareAccelerated)
        {
            return Eval_SIMD(c, x, b);
        }
        return Eval_NoSIMD([.. c], [.. x], b?.ToList());
    }

    internal static List<double> Eval_NoSIMD(List<double> c, List<double> x, List<double>? b = null)
    {
        // if no c, return empty
        if (c.Count == 0)
        {
            return [];
        }

        double[] bias = [];
        if (b == null)
        {
            bias = new double[c.Count - 1];
        }
        else
        {
            bias = [.. b];
            if (bias.Length >= c.Count)
            {
                throw new ArgumentException("total basis numbers must be less than coefficients' numbers-1.");
            }
            if (bias.Length < c.Count - 1)
            {
                Array.Resize(ref bias, c.Count - 1);
            }
        }
        List<double> result = new List<double>(x.Count);
        foreach (var x_i in x)
        {
            double y_i = 0;
            for (int i = c.Count - 1; i > 0; i--)
            {
                y_i = (x_i - bias[i - 1]) * (c[i] + y_i);
            }
            y_i += c[0];
            result.Add(y_i);
        }
        return result;
    }

    internal static List<double> Eval_SIMD(double[] c, double[] x, double[]? b = null)
    {
        // if no c, return empty
        if (c.Length == 0)
        {
            return [];
        }

        double[] bias = [];
        if (b == null)
        {
            bias = new double[c.Length];
        }
        else
        {
            bias = [.. b];
            if (bias.Length >= c.Length)
            {
                throw new ArgumentException("total basis numbers must be less than coefficients' numbers-1.");
            }
            if (bias.Length < c.Length - 1)
            {
                Array.Resize(ref bias, c.Length - 1);
            }
            bias = [0, .. bias];
        }

        int length = x.Length;
        double[] result = new double[length];
        // Get the number of elements that can't be processed in the vector
        // NOTE: Vector<T>.Count is a JIT time constant and will get optimized accordingly
        int remaining = length % Vector<double>.Count;

        for (int i = 0; i < length - remaining; i += Vector<double>.Count)
        {
            Vector<double> y_i = new(0);
            var x_i = new Vector<double>(x, i);
            for (int j = c.Length - 1; j > 0; j--)
            {
                var c_i = new Vector<double>(c[j]);
                var b_i = new Vector<double>(bias[j]);
                y_i = (x_i - b_i) * (c_i + y_i);
            }
            var c_0 = new Vector<double>(c[0]);
            y_i += c_0;
            y_i.CopyTo(result, i);

        }
        for (int i = length - remaining; i < length; i++)
        {
            var x_i = x[i];
            double y_i = 0;
            for (int j = c.Length - 1; j > 0; j--)
            {
                y_i = (x_i - bias[j]) * (c[j] + y_i);
            }
            y_i += c[0];
            result[i] = y_i;
        }
        return [.. result];
    }

}
