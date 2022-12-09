file = File.open(ARGV[0])
moves = file.readlines.map(&:chomp).map(&:split)

def move_head(head, change)
  [head[-1][0] + change[0], head[-1][1] + change[1]]
end

def move_tail(tail, head_position)
  last_tail_position = tail[-1]
  dir = direction(head_position, last_tail_position)
  return last_tail_position if dir.map(&:abs).find{|x| x > 1}.nil? # only one away

  puts dir.inspect

  if dir.min == 0 # only horizontal/vertical
    if dir[0].abs > 1
      [last_tail_position[0]-dir[0]/2, last_tail_position[1]]
    else
      [last_tail_position[0], last_tail_position[1]-dir[1]/2]
    end
  else
    if dir[0].abs > 1
      [last_tail_position[0]-dir[0]/2, last_tail_position[1]-dir[1]]
    else
      [last_tail_position[0]-dir[0], last_tail_position[1]-dir[1]/2]
    end
  end
end

def direction(pos_1, pos_2)
  [pos_2[0]-pos_1[0], pos_2[1]-pos_1[1]]
end

def visualize(tail, head, width, height)
  field = []
  width.times do |x|
    height.times do |y|
      field[y] ||= []
      field[y][x] = '#'
    end
  end
  field[head[1]][head[0]] = "H"
  field[tail[1]][tail[0]] = "T"
  (0..height-1).to_a.reverse.each do |y|
    puts field[y].inspect
  end
  puts
end

head = [[0,0]]
tail = [[0,0]]

count = 0

moves.each do |move|
  change = case move[0]
  when 'R'
    [1,0]
  when 'L'
    [-1,0]
  when 'U'
    [0,1]
  when 'D'
    [0,-1]
  end

  old_h = head[-1]
  old_t = tail[-1]
  move[1].to_i.times do
    head << move_head(head, change)
    tail << move_tail(tail, head[-1])
    #visualize(tail[-1], head[-1], 12, 12)
  end
  #puts "#{move}: #{old_h}->#{head[-1]} - #{old_t}->#{tail[-1]}"
end

#puts tail[0..100].inspect
#puts head[0..100].inspect
#puts tail.uniq.inspect
puts "Result1: #{tail.uniq.size}"

