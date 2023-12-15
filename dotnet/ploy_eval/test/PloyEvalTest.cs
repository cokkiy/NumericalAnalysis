using NUnit.Framework;
using System;
using System.Collections.Generic;
namespace NumericalAnalysis.Tests;

[TestFixture]
public class PloyEvalTests
{
    [Test]
    public void Eval_WithEmptyCoefficients_ReturnsEmptyList()
    {
        List<double> c = new List<double>();
        List<double> x = new List<double> { 1, 2, 3 };
        List<double> expected = new List<double>();

        List<double> result = PloyEval.Eval_NoSIMD(c, x);

        Assert.That(result.Count, Is.EqualTo(0));
    }

    [Test]
    public void Eval_WithNullBias_CalculatesResultCorrectly()
    {
        List<double> c = new List<double> { 1, 2, 3, 4 };
        List<double> x = new List<double> { 1, 2, 3, 4, 5, 7, 8, };
        List<double> expected = new List<double> { 10.0, 49.0, 142.0, 313.0, 586.0, 1534.0, 2257.0, };

        List<double> result = PloyEval.Eval_NoSIMD(c, x);
        CollectionAssert.AreEquivalent(expected, result);
    }

    [Test]
    public void Eval_WithValidBias_CalculatesResultCorrectly()
    {
        List<double> c = new List<double> { 1, 2, 3 };
        List<double> x = new List<double> { 1, 2, 3 };
        List<double> b = new List<double> { 1, 2 };
        List<double> expected = new List<double> { 1, 3, 11 };

        List<double> result = PloyEval.Eval(c, x, b);
        CollectionAssert.AreEquivalent(expected, result);
    }

    [Test]
    public void Eval_WithInvalidBias_ThrowsArgumentException()
    {
        List<double> c = new List<double> { 1, 2, 3 };
        List<double> x = new List<double> { 1, 2, 3 };
        List<double> b = new List<double> { 0, 1, 3 };

        Assert.Throws<ArgumentException>(() => PloyEval.Eval_NoSIMD(c, x, b));
    }


    [Test]
    public void Eval_SIMD_WithEmptyCoefficients_ReturnsEmptyList()
    {
        double[] c = new double[] { };
        double[] x = new double[] { 1, 2, 3 };
        List<double> expected = new List<double>();

        List<double> result = PloyEval.Eval_SIMD(c, x);

        Assert.That(result.Count, Is.EqualTo(0));
    }

    [Test]
    public void Eval_SIMD_WithNullBias_CalculatesResultCorrectly()
    {
        double[] c = new double[] { 1, 2, 3, 4 };
        double[] x = new double[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 };
        List<double> expected = new List<double> { 10.0, 49.0, 142.0, 313.0, 586.0, 985.0, 1534.0, 2257.0, 3178.0, 4321.0, 5710.0, 7369.0, 9322.0, 11593.0, 14206.0, 17185.0, };

        List<double> result = PloyEval.Eval_SIMD(c, x);
        CollectionAssert.AreEquivalent(expected, result);
    }

    [Test]
    public void Eval_SIMD_WithValidBias_CalculatesResultCorrectly()
    {
        double[] c = new double[] { 1, 2, 3 };
        double[] x = new double[] { 1, 2, 3 };
        double[] b = new double[] { 1, 2 };
        List<double> expected = new List<double> { 1, 3, 11 };

        List<double> result = PloyEval.Eval_SIMD(c, x, b);
        CollectionAssert.AreEquivalent(expected, result);
    }

    [Test]
    public void Eval_SIMD_With_EvenX_WithValidBias_CalculatesResultCorrectly()
    {
        double[] c = new double[] { 1.0, 2.0, 3.0, 4.0, 5.0 };
        double[] x = new double[] { 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, };
        double[] b = new double[] { 3.0, 4.0, 5.0, 6.0 };
        List<double> expected = new List<double> { 1.0, 3.0, 11.0, 49.0, 261.0, 911.0, 2383.0, 5181.0, 9929.0, 17371.0, 28371.0, 43913.0,
            65101.0, 93159.0, 129431.0, 175381.0, };

        List<double> result = PloyEval.Eval_SIMD(c, x, b);
        CollectionAssert.AreEquivalent(expected, result);
    }

    [Test]
    public void Eval_SIMD_With_OddX_WithValidBias_CalculatesResultCorrectly()
    {
        double[] c = new double[] { 1.0, 2.0, 3.0, 4.0, 5.0 };
        double[] x = new double[] { 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, };
        double[] b = new double[] { 3.0, 4.0, 5.0, 6.0 };
        List<double> expected = new List<double> { 1.0, 3.0, 11.0, 49.0, 261.0, 911.0, 2383.0, 5181.0, 9929.0, 17371.0, 28371.0, 43913.0,
            65101.0, 93159.0, 129431.0, };

        List<double> result = PloyEval.Eval_SIMD(c, x, b);
        CollectionAssert.AreEquivalent(expected, result);
    }

    [Test]
    public void Eval_SIMD_With_EvenX_WithLessBias_CalculatesResultCorrectly()
    {
        double[] c = new double[] { 1.0, 2.0, 3.0, 4.0, 5.0 };
        double[] x = new double[] { 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, };
        double[] b = new double[] { 3.0, 4.0, 5.0 };
        List<double> expected = new List<double> { -201.0, -79.0, 1.0, 3.0, 11.0, 229.0, 981.0, 2711.0, 5983.0, 11481.0, 20009.0, 32491.0,
            49971.0, 73613.0, };

        List<double> result = PloyEval.Eval_SIMD(c, x, b);
        CollectionAssert.AreEquivalent(expected, result);
    }

    [Test]
    public void Eval_SIMD_With_OddX_WithLessBias_CalculatesResultCorrectly()
    {
        double[] c = new double[] { 1.0, 2.0, 3.0, 4.0, 5.0 };
        double[] x = new double[] { 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, };
        double[] b = new double[] { 3.0, 4.0, 5.0 };
        List<double> expected = new List<double> { -201.0, -79.0, 1.0, 3.0, 11.0, 229.0, 981.0, 2711.0, 5983.0, 11481.0, 20009.0, 32491.0,
            49971.0, 73613.0, 104701.0, };

        List<double> result = PloyEval.Eval_SIMD(c, x, b);
        CollectionAssert.AreEquivalent(expected, result);
    }


    [Test]
    public void Eval_SIMD_WithInvalidBias_ThrowsArgumentException()
    {
        double[] c = new double[] { 1, 2, 3 };
        double[] x = new double[] { 1, 2, 3 };
        double[] b = new double[] { 0, 1, 3 };

        Assert.Throws<ArgumentException>(() => PloyEval.Eval_SIMD(c, x, b));
    }
}