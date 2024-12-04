file = File.open(ARGV[0])
@letters = file.readlines.map(&:chomp).map{ |line| line.split("") }

count = 0

def get(x, y)
  return '' if x < 0
  return '' if y < 0
  return '' if y >= @letters.size
  return '' if x >= @letters[0].size

   @letters[y][x]
end

@letters.each_with_index do |_,y|
  @letters[y].each_with_index do |_,x|
    if get(x,y) == 'X'
      if get(x+1,y)+get(x+2,y)+get(x+3,y) == 'MAS' # right
        count += 1
      end
      if get(x,y+1)+get(x,y+2)+get(x,y+3) == 'MAS' # down
        count += 1
      end
      if get(x-1,y)+get(x-2,y)+get(x-3,y) == 'MAS' # left
        count += 1
      end
      if get(x,y-1)+get(x,y-2)+get(x,y-3) == 'MAS' # up
        count += 1
      end
      if get(x+1,y+1)+get(x+2,y+2)+get(x+3,y+3) == 'MAS' # downright
        count += 1
      end
      if get(x+1,y-1)+get(x+2,y-2)+get(x+3,y-3) == 'MAS' # upright
        count += 1
      end
      if get(x-1,y-1)+get(x-2,y-2)+get(x-3,y-3) == 'MAS' # upleft
        count += 1
      end
      if get(x-1,y+1)+get(x-2,y+2)+get(x-3,y+3) == 'MAS' # downleft
        count += 1
      end
    end
  end
end

puts "Part1: #{count}"

count = 0
@letters.each_with_index do |_,y|
  @letters[y].each_with_index do |_,x|
    if get(x,y) == 'A'
      if get(x-1,y-1) == 'M' && get(x+1,y+1) == 'S' && get(x-1,y+1) == 'M' && get(x+1,y-1) == 'S'
        # M S
        #  A
        # M S
        count += 1
      end
      if get(x-1,y-1) == 'S' && get(x+1,y+1) == 'M' && get(x-1,y+1) == 'M' && get(x+1,y-1) == 'S'
        # S S
        #  A
        # M M
        count += 1
      end
      if get(x-1,y-1) == 'S' && get(x+1,y+1) == 'M' && get(x-1,y+1) == 'S' && get(x+1,y-1) == 'M'
        # S M
        #  A
        # S M
        count += 1
      end
      if get(x-1,y-1) == 'M' && get(x+1,y+1) == 'S' && get(x-1,y+1) == 'S' && get(x+1,y-1) == 'M'
        # M M
        #  A
        # S S
        count += 1
      end
    end
  end
end

puts "Part2: #{count}"