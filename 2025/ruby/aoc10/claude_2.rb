require 'set'

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).map do |l|
  parts = l.split(' ')
  target = parts[0][1..-2].gsub('#', '1').gsub('.', '0').to_i(2)
  buttons = parts[1..-2].map do |b|
    b[1..-2].split(',').map(&:to_i)
  end
  joltage = parts[-1][1..-2].split(',').map(&:to_i)
  [target, buttons, joltage, l]
end

def solve(buttons, target, size)
  return 0 if target.all?(&:zero?)

  # Convert buttons to delta vectors
  deltas = buttons.map do |b|
    delta = Array.new(size, 0)
    b.each { |i| delta[i] += 1 }
    delta
  end

  # We need to find non-negative integers x[0], x[1], ... x[n-1] such that:
  # sum(x[i] * deltas[i][j]) = target[j] for all j
  # minimize sum(x[i])

  # Use iterative deepening with pruning
  # Heuristic: minimum presses needed = max(ceil(target[j] / max_contribution_to_j))
  max_contrib = Array.new(size, 0)
  size.times do |j|
    deltas.each { |d| max_contrib[j] = [max_contrib[j], d[j]].max }
  end

  min_presses = target.each_with_index.map { |t, j| max_contrib[j] > 0 ? (t.to_f / max_contrib[j]).ceil : (t > 0 ? Float::INFINITY : 0) }.max

  # DFS with memoization and pruning
  # State: remaining target to fill
  # Use iterative deepening

  best = Float::INFINITY

  # Sort deltas by "efficiency" - total contribution per press
  sorted_indices = (0...deltas.size).sort_by { |i| -deltas[i].sum }
  sorted_deltas = sorted_indices.map { |i| deltas[i] }

  def dfs(remaining, deltas, idx, presses, best_ref, memo_key)
    return presses if remaining.all?(&:zero?)
    return Float::INFINITY if idx >= deltas.size
    return Float::INFINITY if presses >= best_ref[0]

    # Pruning: check if it's even possible
    remaining.each_with_index do |r, j|
      if r > 0
        can_fill = false
        (idx...deltas.size).each do |i|
          if deltas[i][j] > 0
            can_fill = true
            break
          end
        end
        return Float::INFINITY unless can_fill
      end
    end

    delta = deltas[idx]

    # How many times can we use this button? (limited by remaining values)
    max_uses = remaining.each_with_index.map { |r, j| delta[j] > 0 ? r / delta[j] : Float::INFINITY }.min
    max_uses = [max_uses, 500].min  # cap to avoid infinite loops

    result = Float::INFINITY

    # Try from max down to 0 (greedy first)
    max_uses.downto(0) do |uses|
      new_remaining = remaining.map.with_index { |r, j| r - delta[j] * uses }
      next if new_remaining.any?(&:negative?)

      sub_result = dfs(new_remaining, deltas, idx + 1, presses + uses, best_ref, nil)
      if sub_result < result
        result = sub_result
        best_ref[0] = [best_ref[0], result].min
      end

      # Early exit if we found optimal for this branch
      break if result == presses + uses + new_remaining.map.with_index { |r, j|
        max_c = (idx+1...deltas.size).map { |i| deltas[i][j] }.max || 0
        max_c > 0 ? (r.to_f / max_c).ceil : (r > 0 ? 9999 : 0)
      }.max
    end

    result
  end

  best_ref = [Float::INFINITY]
  result = dfs(target.dup, sorted_deltas, 0, 0, best_ref, nil)

  result == Float::INFINITY ? nil : result
end

sum = 0
data.each do |d|
  result = solve(d[1], d[2], d[2].size)
  if result
    puts "Found #{result} for #{d[3]}"
    sum += result
  else
    puts "No solution for #{d[3]}"
  end
end

puts sum
