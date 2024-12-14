require "set"

file = File.open(ARGV[0])
robots = file.readlines.map(&:chomp).map do |line|
  match = line.match(/p=(\d*),(\d*) v=(-?\d*),(-?\d*)/)
  [[match[1].to_i,match[2].to_i],[match[3].to_i,match[4].to_i]]
end

if ARGV[0] =~ /_test/
  width = 11
  height = 7
else
  width = 101
  height = 103
end

times = 100
result_1 = robots.map do |robot|
  [(robot[0][0]+times*robot[1][0])%width, (robot[0][1]+times*robot[1][1])%height]
end

def print_grid(width, height, result)
  height.times do |y|
    width.times do |x|
      if result.include?([x,y])
        putc result.select{|res| res == [x,y]}.size.to_s
      else
        putc "."
      end
    end
    puts
  end
end

print_grid(width, height, result_1)

# top left
result = result_1.select{|res| res[0] < width/2 && res[1] < height/2}.size
# top right
result *= result_1.select{|res| res[0] > width/2 && res[1] < height/2}.size
# bottom left
result *= result_1.select{|res| res[0] < width/2 && res[1] > height/2}.size
# bottom right
result *= result_1.select{|res| res[0] > width/2 && res[1] > height/2}.size

puts "Part1: #{result}"

100000.times do |times|
  result = robots.map do |robot|
    [(robot[0][0]+times*robot[1][0])%width, (robot[0][1]+times*robot[1][1])%height]
  end

  if result.size == result.uniq.size
    print_grid(width, height, result)
    puts "Part2: #{times}"
    break
  end
end