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
    public static List<double> Eval(List<double> c, List<double> x, List<double>? b = null)
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
}
