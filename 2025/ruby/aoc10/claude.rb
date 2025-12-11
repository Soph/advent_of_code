def bidirectional_bfs(buttons, target, size)
  # Forward: states reachable from start
  forward = { Array.new(size, 0) => 0 } # state => steps
  forward_frontier = Set.new([Array.new(size, 0)])

  # Backward: states that can reach target
  # We need to think in reverse: what states could have led to target?
  backward = { target => 0 }
  backward_frontier = Set.new([target])

  # Precompute button deltas
  deltas = buttons.map do |b|
    delta = Array.new(size, 0)
    b.each { |i| delta[i] += 1 }
    delta
  end

  forward_steps = 0
  backward_steps = 0

  loop do
    # Expand the smaller frontier
    if forward_frontier.size <= backward_frontier.size
      forward_steps += 1
      new_frontier = Set.new

      forward_frontier.each do |state|
        deltas.each do |delta|
          new_state = state.zip(delta).map { |a, b| a + b }
          next if new_state.zip(target).any? { |a, b| a > b } # pruning

          return forward_steps + backward[new_state] if backward.key?(new_state)

          unless forward.key?(new_state)
            forward[new_state] = forward_steps
            new_frontier << new_state
          end
        end
      end
      forward_frontier = new_frontier
    else
      # Expand backward (subtract deltas)
      backward_steps += 1
      new_frontier = Set.new

      backward_frontier.each do |state|
        deltas.each do |delta|
          new_state = state.zip(delta).map { |a, b| a - b }
          next if new_state.any?(&:negative?) # can't go below 0

          return forward[new_state] + backward_steps if forward.key?(new_state)

          unless backward.key?(new_state)
            backward[new_state] = backward_steps
            new_frontier << new_state
          end
        end
      end
      backward_frontier = new_frontier
    end

    return nil if forward_frontier.empty? && backward_frontier.empty?
  end
end

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

sum = 0
data.each do |d|
  sum += bidirectional_bfs(d[1], d[0], d[2].size)
end
