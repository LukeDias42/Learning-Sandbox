### Question 2:
Find the shortest sequence of moves that transfers a tower of _n_ disks from the left peg A to the right peg B, if direct moves between A and B are disallowed. (Each move must be to or from the middle peg. As usual, a larger disk must never appear above a smaller one.)

## Answer:
Every time we want to move a disc N to the middle, we have to completely solve N-1 discs, then we need to solve it again from B to A, move the Nth disc, and solve it again for N-1 discs.

So:
$$T_N = 3T_{N-1} + 2$$
Induction step:
$$
T_N = 3(3^{N-1}-1) + 2\\
T_N = 3^N-3 + 2 = 3^N - 1\\
\text{$\:$ Or $\: T_N + 1 = 3T_{N-1}+3$}\\
\text{$\:$ Considering $\: U_0 = 1 \:$ then $\: U_N = 3U_{N-1}\Rightarrow3^N$}\\
U_N = 3N\Rightarrow T_N + 1 = 3^N\Rightarrow T_N=3^N-1\\
T_N = 3^{N-1}
$$

### Question 3:
Showthat, in the process of transferring a tower under the restrictions of the preceding exercise, we will actually encounter every properly stacked arrangement of n disks on three pegs.

### Answer
Every disc has 3 possible positions, peg A, middle peg and peg B, so the amount of different configuration for all discs are $3\times3\times3\times..\times3 = 3^N$, considering that you never repeat a configuration when moving the discs, and adding the first configuration at the start, we have that the amount of configurations that you go through and the amount of possible configurations are the same.