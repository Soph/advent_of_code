require "set"

file = File.open(ARGV[0])
data = {}
file.readlines.map(&:chomp).map do |l|
  part = l.split(": ")
  data[part[0]] = part[1].split(" ")
end

# Get topological order
def topo_sort(data)
  in_degree = Hash.new(0)
  all_nodes = Set.new

  data.each do |from, tos|
    all_nodes << from
    tos.each do |to|
      all_nodes << to
      in_degree[to] += 1
    end
  end

  queue = all_nodes.select { |n| in_degree[n] == 0 }.to_a
  sorted = []

  while !queue.empty?
    node = queue.shift
    sorted << node
    (data[node] || []).each do |neighbor|
      in_degree[neighbor] -= 1
      queue << neighbor if in_degree[neighbor] == 0
    end
  end
  sorted
end

topo = topo_sort(data)

# DP: dp[node][state] = number of paths from svr to node with checkpoint state
# state: 0 = neither, 1 = dac only, 2 = fft only, 3 = both
CHECKPOINTS = { "dac" => 1, "fft" => 2 }

dp = Hash.new { |h, k| h[k] = [0, 0, 0, 0] }
dp["svr"][0] = 1

topo.each do |node|
  # Update state when entering this node
  bit = CHECKPOINTS[node] || 0

  next_states = dp[node].map.with_index do |count, state|
    [count, state | bit]
  end

  (data[node] || []).each do |neighbor|
    next_states.each do |count, new_state|
      dp[neighbor][new_state] += count
    end
  end
end

puts "Paths to out with both dac and fft: #{dp["out"][3]}"
