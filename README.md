

# Advent of Code 2018 - Solutions
This repository contains my solutions to [Advent of Code 2018](https://adventofcode.com/2018) written in Rust.

Solutions for each day can be found in `src/bin/`, where each file is a standalone executable. Common code is separated into modules and stored in separate subfolders inside `src/bin/`.

Input data for each solution is in `res/input`, while misc files are in `res/other`.

## Days
Below is an overview of each day's solution.

### Day 1:  Chronal Calibration
* **Part 1**: Simply sum the frequencies and print the result.  
`⏳O(n)` | `📦O(1)`, where n=number of frequencies.
* **Part 2**: Sum frequencies in an infinite cycle and consult a HashSet of seen values until a repetition is found.  
`⏳O(n)` | `📦O(n)`, where n=number of frequencies.

### Day 2:  Inventory Management System
* **Part 1**: Count boxes ids with 2 and 3 duplicate letters.  
`⏳O(n)` | `📦O(m)`, where n=number of boxes, and m=length of box ids.
* **Part 2 (v1)**: Sort box ids; ignoring each character position in turn.
Box ids differing by only a single character will be sorted adjacent, and can be then found by a linear scan.  
`⏳O(n·log(n)·m²)` | `📦O(n + m)`, where n=number of boxes, and m=length of box ids.
* **Part 2 (v2)**: Check all pairs of box ids (n choose 2 combinations) to see if they differ by exactly one character.  
`⏳O(n²·m)` | `📦O(m)`, where n=number of boxes, and m=length of box ids.

### Day 3: No Matter How You Slice It
* **Part 1**: Mark all rectangular claims in a fixed array (1000²) of coordinates; incrementing the claim count for each coordinate. Finally, count those which have been claimed more than once.  
`⏳O(n·m)` | `📦O(n)`, where n=number of claims, and m=size of each claim.
* **Part 2**: Mark claims as above, then check the area of each claim to find which has no overlap.  
`⏳O(n·m)` | `📦O(n)`, where n=number of claims, and m=size of each claim.

### Day 4: Repose Record
* **Part 1 (v1)**: Parse each guard event as one of (Shift Change, Wake, Sleep), then group events by shift into chronological order. Finally, sum minutes asleep for each guard and sleep totals for each minute.  
`⏳O(n)` | `📦O(n)`, where n=number of events.
* **Part 1 (v2)**:  Similar to above, but explicitly mark each minute of each shift as awake or asleep when reading in shift events, rather than only storing the events.  
`⏳O(n)` | `📦O(n)`, where n=number of events.
* **Part 2** Parse each guard event as in Part 1 (v1), then sum minutes asleep for each guard, and finally find which which guard is most frequently asleep on the same minute.  
`⏳O(n)` | `📦O(n)`, where n=number of events.

### Day 5: Alchemical Reduction
* **Part 1**: Load polymer into a sparse vector (for efficient removal), then continually search for adjacent unit pairs and remove them until there are no further reactions.  
`⏳O(n²)` | `📦O(n)`, where n=length of the polymer.
* **Part 2 (v1)**:  Similar to Part 1, except try with every unit type removed to see which results in the smallest polymer after being fully reacted.  
`⏳O(n²)` | `📦O(n)`, where n=length of the polymer.
* **Part 2 (v2)**:  Similar to Part 2 (v1), except use an iterator and stack to allow efficient reacting and removal. This results in only requiring one pass over the polymer. In addition, the original polymer is fully reacted before being used as a base for each round of unit removal; thus removing redundant operations.  
`⏳O(n)` | `📦O(n)`, where n=length of the polymer.

### Day 6: Chronal Coordinates
* **Part 1**: Start by defining a bounding rectangle that contains all points, then for each coordinate in that rectangle we check it's distance to every point and increment the closest point's counter. Coordinates at the edge of the bounding rectangle are part of an infinite area and are not considered. At the end, the highest count for any point is the answer. This problem is essentially a [Voronoi diagram](https://en.wikipedia.org/wiki/Voronoi_diagram) (see [visualisation]()), and a technique such as [Fortune's Algorithm](https://en.wikipedia.org/wiki/Fortune%27s_algorithm) would be much more efficient, although considerably more complicated to implement.  
`⏳O(n·m)` | `📦O(n)`, where n=number of points, and m=size area to contain all points.
* **Part 2**: Start by averaging all points to get a "center" point. Then spiral outwards from that point and count the number of coordinates whose sum of distances to all other points is < 10,000. We stop once a full layer of the spiral has completed without seeing any valid coordinates. The size of this area is [*approximately*](https://i.imgur.com/YrQhIHI.png) circular and centered near this "center" point, although not quite enough to use a purely mathematical formula to solve this problem.  
`⏳O(n·m)` | `📦O(1)`, where n=number of points, and m=size area to contain all points.

### Day 7: The Sum of Its Parts
* **Part 1**: Start by gathering a list of steps and their dependencies (if any), then iteratively search through the list of steps for the next step without any dependencies. As a step is completed, it is removed as a dependency from from all other steps. This is repeated until all steps are complete.  
`⏳O(n²)` | `📦O(n·m)`, where n=number of steps.
* **Part 2**: Start by gathering a list of steps and their dependencies (if any), then iteratively search for and assign steps without dependencies to available workers. Once there is no more work or workers, we jump forward in time to the next completed step and mark it complete as in Part 1. This is repeated until all steps are complete.  
`⏳O(n²)` | `📦O(n·m)`, where n=number of steps.

### Day 8: Memory Maneuver
* **Part 1**: Recursively calculate size and metadata for each nested child. The size of a child node is used to find the offset to the next child node (in case of multiple children) and to the metadata entries. The position and value of all metadata entries is then known and the sum can be taken.  
`⏳O(n + m)` | `📦O(log(n))`, where n=number of child nodes, and m=number of metadata entries.
* **Part 2**: Recursively calculate size and value for each nested child. If a metadata entry is a valid index to a child node (1-based), then add it's value, otherwise add the raw metadata entry. This is used recursively to calculate the value of the root node.  
`⏳O(n + m)` | `📦O(log(n))`, where n=number of child nodes, and m=number of metadata entries.

### Day 9: Marble Mania
* **Part 1 & 2**: The game board is stored in a circular double-linked list backed by an array with the current marble tracked by a cursor. This allows efficient traversal `O(k)`, insertion `O(1)`, and removal `O(1)` of marbles as the game progresses.  
`⏳O(n)` | `📦O(n)`, where n=number of marbles/turns.

### Day 10: The Stars Align
* **Part 1 & 2 (v1)**: Use an Equal Interval search along with a heuristic function to find the time when all points converge. The heuristic function calculates the area of a bounding box required to fit all points at the given time. After approx. 28 iterations, the time of convergence is known and the position of all points at that time are rendered into the final message.  
`⏳O(n·log(m))` | `📦O(n)`, where n=number of points, and m=time to convergence.
* **Part 1 & 2 (v2)**: Calculate the time of intersection between multiple pairs of points, take the average, and round to the nearest integer. The points are then moved to this time step and their positions are rendered into the final message.  
`⏳O(n)` | `📦O(n)`, where n=number of points.

### Day 11: Chronal Charge
* **Part 1 (v1)**: Generate 300x300 matrix of fuel cell values, then sum the values of every possible 3x3 submatrix.  
`⏳O(n²·m²)` | `📦O(n²)`, where n=dimensions of matrix, and m=dimensions of submatrix.
* **Part 1 (v2)**: Iterate over all coordinates of the 300x300 matrix. For each, we calculate and save the both the power level and the sum of 3 power levels to the left (inclusive). We then sum the 3 power level sums above each coordinate to get us the sum of values of every 3x3 submatrix in `2m` steps.  
`⏳O(n²·m)` | `📦O(n²)`, where n=dimensions of matrix, and m=dimensions of submatrix.
* **Part 1 (v3)**: Generate a [summed-area table](https://en.wikipedia.org/wiki/Summed-area_table) for the 300x300 matrix; where each coordinate contains the sum of all coordinates above and to the left (inclusive). Using this, we can calculate the sum of any submatrix in constant time using the inclusion-exclusion principle. We then iterate over each coordinate of the matrix and sum the 3x3 submatrix anchored there. The coordinate of the submatrix with the largest sum is remembered as the final answer.  
`⏳O(n²)` | `📦O(n²)`, where n=dimensions of matrix.
* **Part 2**: Same as Part 1 (v3), except we look at all square submatrices anchored at their lower-right whose dimensions fit within the bounds of the matrix.  
`⏳O(n³)` | `📦O(n²)`, where n=dimensions of matrix.

>TODO: Complete the rest of the challenges.
