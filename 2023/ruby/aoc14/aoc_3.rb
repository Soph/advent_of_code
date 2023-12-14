require "Set"
file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)
grid = lines.map(&:chars)

rocks = Set.new
static_rocks = Set.new
grid.each_with_index do |line, y|
  line.each_with_index do |rock, x|
    if rock == 'O'
      rocks << [x,y]
    end
    if rock == '#'
      static_rocks << [x,y]
    end
  end
end

def calc_weight(rocks, max_y)
  sum = 0
  rocks.each do |rock|
    sum += max_y - rock[1]
  end
  sum
end

def print_grid(rocks, static_rocks, max_x, max_y)
  (0...max_y).each do |y|
    (0...max_x).each do |x|
      if static_rocks.include?([x,y])
        putc '#'
      elsif rocks.include?([x,y])
        putc 'O'
      else
        putc '.'
      end
    end
    puts
  end
end

cache = {}
directions = [[-1,0],[0,-1],[1,0],[0,1]]
count = 1000000000
n = 0
weights_after_cache = Set.new
first_cached_state = nil
first_n_for_cached_state = 0
while n < count do
  state = rocks.clone
  if cache[state]
    if first_cached_state && first_cached_state == state
      # we are caching, and we repeated
      repitition = n-first_n_for_cached_state
      puts "repeating for #{n-first_n_for_cached_state}"
      rest = count - n
      n = count - (rest % repitition)
    end
    if !first_cached_state
      first_cached_state ||= state.clone
      first_n_for_cached_state = n
    end
    #puts rocks.inspect
    rocks = cache[state]
    puts "#{n}: #{calc_weight(rocks, grid.size)}"
    #puts rocks.inspect
    #sleep 10
  else
    directions.each do |direction|
      found = true
      while found
        found = false
        checks = []
        case direction
        when [-1,0]
          checks = rocks.sort_by {|rock_a, rock_b| rock_a[1] <=> rock_b[1]}
        when [1, 0]
          checks = rocks.sort_by {|rock_a, rock_b| rock_a[1] <=> rock_b[1]}.reverse
        when [0, -1]
          checks = rocks.sort_by {|rock_a, rock_b| rock_a[0] <=> rock_b[0]}
        when [0, 1]
          checks = rocks.sort_by {|rock_a, rock_b| rock_a[0] <=> rock_b[0]}.reverse
        end
        checks.each do |rock|
          target = [rock[0] + direction[1], rock[1] + direction[0]]
          next if target[1] < 0 || target[1] >= grid.size # y out of bounds
          next if target[0] < 0 || target[0] >= grid[0].size # x out of bounds
          next if static_rocks.include?(target) # there is a static rock
          next if rocks.include?(target) # there is a rolling rock

          #puts "Move: #{rock} -> #{target}"
          rocks.delete(rock)
          rocks << target
          found = true
        end
        #puts rocks.inspect
        #puts calc_weight(rocks, grid.size)
      end
    end
    cache[state] ||= rocks.clone
  end
  #print_grid(rocks, static_rocks, grid[0].size, grid.size)
  puts "#{n}: #{calc_weight(rocks, grid.size)}" if n % 10000 == 9999

  n += 1
end
