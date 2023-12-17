require "Set"

file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map(&:chars).map{|row| row.map(&:to_i)}

start = [0,0]
moves = []
# pos, dir, times in a direction, heat loss, steps
moves << [[0,0], [1,0], 0, 0, 0]
moves << [[0,0], [0,1], 0, 0, 0]

directions = [[0,1], [1,0], [0,-1], [-1,0]]
known = {}

i = 0
min_finish_heat = 1000000000
min_finish = nil
while !moves.empty?
  i+=1
  move = moves.shift

  known[[move[0],move[1],move[2]]] ||= 1000000000
  if known[[move[0],move[1],move[2]]] > move[3]
    known[[move[0],move[1],move[2]]] = move[3]
  else
    next # we have a quicker path to here going
  end
  directions.each do |direction|
    new_pos = [move[0][0] + direction[0], move[0][1] + direction[1]]
    next if new_pos[0] < 0 || new_pos[0] >= grid[0].size # out of bounds x
    next if new_pos[1] < 0 || new_pos[1] >= grid.size # out of bounds y
    next if [move[1][0]+direction[0],move[1][1]+direction[1]] == [0,0] # opposite direction

    steps_direction = move[2]
    if direction == move[1] # continue same direction
      steps_direction += 1
    else
      steps_direction = 0
    end
    next if steps_direction >= 3 # to much, not possible
    new_heat = move[3] + grid[new_pos[1]][new_pos[0]]
    next if new_heat > min_finish_heat
    if new_pos == [grid[0].size - 1, grid.size - 1]
      # finish
      if min_finish_heat > new_heat
        puts "Finished: #{[new_pos, direction, steps_direction, new_heat].inspect}"
        finish = [new_pos, direction, steps_direction, new_heat, move[4]+1]
        min_finish_heat = new_heat
      end
    else
      moves << [new_pos, direction, steps_direction, new_heat, move[4]+1]
    end
  end
  moves.uniq!
  moves.sort_by!{|move| move[3]}
  puts moves.size if i%100 == 0
end