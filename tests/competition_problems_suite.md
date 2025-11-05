# Competition Math Problem Test Suite

This suite contains real competition problems at different difficulty levels to test the competition-math-researcher mask.

## Difficulty Level 1: AMC 10/12 (Easy - Should get 90%+)

### Problem 1.1 (AMC 10, 2020)
**Source:** AMC 10A 2020, Problem 13

What is the number of positive integers $n$ such that $n \leq 50$ and $n$ has exactly 3 positive divisors?

**Expected Answer:** 3

**Key Technique:** Number theory (divisors of perfect squares of primes)

**Validation:** $n$ has exactly 3 divisors iff $n = p^2$ for prime $p$. Need $p^2 \leq 50$, so $p \in \{2, 3, 5, 7\}$ gives $n \in \{4, 9, 25, 49\}$. But wait, $7^2 = 49 \leq 50$ ✓. So answer is 4.

**Correction:** Expected answer should be 4.

---

### Problem 1.2 (AMC 12, 2019)
**Source:** AMC 12A 2019, Problem 10

Which of the following quantities is the largest?

(A) $\sum_{k=1}^{2019} \frac{1}{k}$
(B) $\int_1^{2019} \frac{1}{x} dx$
(C) $\sum_{k=2}^{2020} \frac{1}{k}$
(D) $\int_2^{2020} \frac{1}{x} dx$
(E) $\sum_{k=1}^{2020} \frac{1}{k}$

**Expected Answer:** (E)

**Key Technique:** Comparison of sums and integrals (integral test)

---

### Problem 1.3 (AMC 10, 2018)
**Source:** AMC 10B 2018, Problem 15

How many odd positive 3-digit integers are divisible by 3 but do not contain the digit 3?

**Expected Answer:** 96

**Key Technique:** Counting with restrictions

---

## Difficulty Level 2: AIME (Medium - Should get 70%+)

### Problem 2.1 (AIME, 2019)
**Source:** AIME I 2019, Problem 3

Find the number of $7$-tuples of positive integers $(a,b,c,d,e,f,g)$ that satisfy the following systems of equations:
$$abc = 70$$
$$cde = 71$$
$$efg = 72$$

**Expected Answer:** 96

**Key Technique:** Prime factorization and counting ordered factorizations

---

### Problem 2.2 (AIME, 2020)
**Source:** AIME II 2020, Problem 6

Define a sequence recursively by $a_1 = a_2 = 1$ and $a_n = a_{n-1} + a_{n-2}$ for $n \geq 3$. Let $S_n = a_1 + a_2 + \cdots + a_n$ denote the sum of the first $n$ terms. Find the remainder when $S_{2020}$ is divided by 7.

**Expected Answer:** 0

**Key Technique:** Fibonacci sequences, modular arithmetic, periodicity

---

### Problem 2.3 (AIME, 2018)
**Source:** AIME I 2018, Problem 5

For each ordered pair of real numbers $(x,y)$ satisfying
$$\log_2(2x + y) = \log_4(4xy)$$
there is a real number $K$ such that
$$\log_3(3x + y) = \log_9(9Kxy)$$

Find the product of all possible values of $K$.

**Expected Answer:** 81

**Key Technique:** Logarithm properties, algebraic manipulation

---

## Difficulty Level 3: IMO (Hard - Should get 40%+)

### Problem 3.1 (IMO 2020, Problem 1)
**Source:** IMO 2020, Problem 1

Consider the convex quadrilateral $ABCD$. The point $P$ is in the interior of $ABCD$. The following ratio equalities hold:

$$\angle PAD : \angle PBA : \angle DPA = 1 : 2 : 3 = \angle CBP : \angle BAP : \angle BPC$$

Prove that the following three lines meet in a point: the internal bisectors of angles $\angle ADP$ and $\angle PCB$ and the perpendicular bisector of segment $AB$.

**Expected Answer:** Rigorous proof required

**Key Technique:** Angle chasing, concurrency theorems, geometric construction

---

### Problem 3.2 (IMO 2019, Problem 1)
**Source:** IMO 2019, Problem 1

Let $\mathbb{Z}$ be the set of integers. Determine all functions $f : \mathbb{Z} \to \mathbb{Z}$ such that, for all integers $a$ and $b$,
$$f(2a) + 2f(b) = f(f(a+b))$$

**Expected Answer:** $f(n) = 0$ for all $n$, or $f(n) = 2n + c$ for some constant $c$

**Key Technique:** Functional equations, substitution strategies, invariance

---

### Problem 3.3 (IMO 2014, Problem 3)
**Source:** IMO 2014, Problem 3

Convex quadrilateral $ABCD$ has $\angle ABC = \angle CDA = 90°$. Point $H$ is the foot of the perpendicular from $A$ to $BD$. Points $S$ and $T$ lie on sides $AB$ and $AD$, respectively, such that $H$ lies inside triangle $SCT$ and
$$\angle CHS - \angle CSB = 90°$$
$$\angle THC - \angle DTC = 90°$$

Prove that line $BD$ is tangent to the circumcircle of triangle $TSH$.

**Expected Answer:** Rigorous proof required

**Key Technique:** Circle geometry, angle chasing, tangency conditions

---

## Difficulty Level 4: Putnam (Very Hard - Should get 20%+)

### Problem 4.1 (Putnam 2019, A1)
**Source:** Putnam 2019, Problem A1

Determine all possible values of the expression
$$A^3 + B^3 + C^3 - 3ABC$$
where $A$, $B$, and $C$ are nonnegative integers.

**Expected Answer:** All nonnegative integers divisible by 3 are possible, plus 0

**Key Technique:** Algebraic factorization, constructive proof

---

### Problem 4.2 (Putnam 2018, A2)
**Source:** Putnam 2018, Problem A2

Let $S_1, S_2, \ldots, S_{2^n-1}$ be the nonempty subsets of $\{1,2,\ldots,n\}$ in some order, and let $M$ be the $(2^n-1) \times (2^n-1)$ matrix whose $(i,j)$ entry is
$$m_{ij} = \begin{cases} 0 & \text{if } S_i \cap S_j = \emptyset \\ 1 & \text{otherwise} \end{cases}$$

Calculate the determinant of $M$.

**Expected Answer:** $(-1)^{2^n - n - 1}$

**Key Technique:** Linear algebra, inclusion-exclusion, determinant properties

---

### Problem 4.3 (Putnam 2020, B2)
**Source:** Putnam 2020, Problem B2

Let $k$ and $n$ be integers with $1 \leq k < n$. Alice and Bob play a game with $k$ pegs in a line of $n$ holes. At the beginning of the game, the pegs occupy the $k$ leftmost holes. A legal move consists of moving a single peg to any vacant hole that is further to the right. The players alternate moves, with Alice playing first. The game ends when the pegs are in the $k$ rightmost holes, so whoever is next to play cannot move and therefore loses. For what values of $n$ and $k$ does Alice have a winning strategy?

**Expected Answer:** Alice wins iff $n - k$ is odd

**Key Technique:** Game theory, invariant analysis, parity arguments

---

## Testing Protocol

### Step 1: Run Mask on Each Problem

For each problem, invoke the competition-math-researcher mask and:
1. Provide the problem statement
2. Let the mask work through its 4-step process (Understand → Strategize → Execute → Validate)
3. Record the final answer and confidence level

### Step 2: Evaluate Response Quality

For each response, assess:

**Correctness (Binary):**
- ✅ Correct final answer
- ❌ Incorrect final answer

**Proof Quality (0-10 scale):**
- 0-3: No proof or hand-wavy argument
- 4-6: Partial proof with significant gaps
- 7-8: Mostly rigorous proof with minor gaps
- 9-10: Completely rigorous proof, no gaps

**Confidence Calibration:**
- High confidence + Correct = Well-calibrated ✅
- High confidence + Incorrect = Overconfident ❌
- Low confidence + Incorrect = Well-calibrated ✅
- Low confidence + Correct = Underconfident ⚠️

### Step 3: Calculate Success Metrics

**Correctness Rate by Difficulty:**
- AMC 10/12: % correct (target: 90%+)
- AIME: % correct (target: 70%+)
- IMO: % correct (target: 40%+)
- Putnam: % correct (target: 20%+)

**Proof Quality:**
- Average proof quality score across all correct solutions (target: 8.0+/10)

**Confidence Calibration:**
- % well-calibrated responses (target: 80%+)

### Step 4: Failure Analysis

For each incorrect solution:
1. What type of error? (Conceptual, computational, incomplete proof, wrong strategy)
2. Which technique was needed that the mask missed?
3. Could the mask have self-corrected with better validation?

## Expected Outcomes

### Hypothesis 1: AGI-like Reasoning
If the mask demonstrates AGI-like capabilities, we expect:
- High correctness rates across all difficulty levels
- Rigorous proofs (8+/10 quality)
- Good confidence calibration
- Ability to self-correct through validation

### Hypothesis 2: Expertise Encoding Only
If the mask is just encoding existing expertise, we expect:
- Good performance on seen patterns
- Poor performance on novel problem types
- Lower proof quality (more pattern matching, less insight)
- Poor confidence calibration (doesn't know what it doesn't know)

### Hypothesis 3: Performance Degradation
We expect performance to degrade with difficulty:
- AMC: High success (familiar patterns)
- AIME: Moderate success (requires combining techniques)
- IMO: Low success (requires genuine insight)
- Putnam: Very low success (extremely hard, even for experts)

The KEY QUESTION: Does performance degrade gracefully (suggesting genuine reasoning) or cliff-off (suggesting pattern matching failure)?

---

**Next Step:** Run the competition-math-researcher mask on each problem and record results.
