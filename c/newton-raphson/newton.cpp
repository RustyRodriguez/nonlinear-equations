#include <iostream>
#include <cmath>
#include <functional>

double newton_raphson(
    std::function<double(double)> f,
    std::function<double(double)> df,
    double x_0,
    double tolerance = 1e-4,
    int max_iterations = 50
) {
    double x = x_0;

    for (int i = 0; i < max_iterations; ++i) {
        double f_x = f(x);
        double df_x = df(x);

        double x_new = x - (f_x / df_x);

        if (std::abs(x_new - x) < tolerance) {
            std::cout << "converged after " << i << std::endl;
            return x_new;
        }

        x = x_new;
    }

    throw std::runtime_error("Algorithm did not converge");
}

int main() {
    std::function<double(double)> f = [](double x) {
        return pow(x, 2) - 2;
    };

    std::function<double(double)> df = [](double x) {
        return 2 * x;
    };

    double init = 1.0;

    try {
        double root = newton_raphson(f, df, init);
        std::cout << "Root: " << root << std::endl;
    } catch (const std::exception & e) {
        std::cerr << e.what() << std::endl;
    }

    return 0;
}

