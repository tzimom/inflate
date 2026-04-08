import sys
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt


def main():
    plt.ion()
    _, balance_ax = plt.subplots()
    daily_budget_ax = balance_ax.twinx()

    balance, = balance_ax.plot([], [], label="balance", color="blue")
    daily_budget, = daily_budget_ax.plot([], [], label="daily budget", color="orange")

    balance_ax.legend([balance, daily_budget], [balance.get_label(), daily_budget.get_label()])

    all_t = []
    all_balance = []
    all_daily_budget = []

    t = 0
    reader = pd.read_csv(sys.stdin, chunksize=100)

    for chunk in reader:
        t_space = np.linspace(t, t + len(chunk) - 1, len(chunk))

        all_t.extend(t_space)
        all_balance.extend(chunk["balance"])
        all_daily_budget.extend(chunk["daily_budget"])

        balance.set_data(all_t, all_balance)
        daily_budget.set_data(all_t, all_daily_budget)

        balance_ax.relim()
        balance_ax.autoscale_view()
        daily_budget_ax.relim()
        daily_budget_ax.autoscale_view()

        plt.draw()
        plt.pause(0.01)

        t += len(chunk)

    plt.ioff()
    plt.show()


if __name__ == "__main__":
    main()
