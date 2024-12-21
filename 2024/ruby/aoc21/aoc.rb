require "set"

file = File.open(ARGV[0])
numbers = file.readlines.map(&:chomp)

puts numbers.inspect

@key_mapping_coordinates = {
  "7" => [0,0],
  "8" => [1,0],
  "9" => [2,0],
  "4" => [0,1],
  "5" => [1,1],
  "6" => [2,1],
  "1" => [0,2],
  "2" => [1,2],
  "3" => [2,2],
  "0" => [1,3],
  "A" => [2,3]
}

@arrow_mapping_coordinates = {
  "^" => [1,0],
  "A" => [2,0],
  "<" => [0,1],
  "v" => [1,1],
  ">" => [2,1]
}

def valid?(start, moves, forbidden)
  pos = start
  moves.each do |move|
    case move
    when ">"
      pos = [pos[0]+1,pos[1]]
    when "<"
      pos = [pos[0]-1,pos[1]]
    when "^"
      pos = [pos[0],pos[1]-1]
    when "v"
      pos = [pos[0],pos[1]+1]
    end
    return false if pos == forbidden
  end
  return true
end

@cache = {}
def move(start, keys_variants, mapping, forbidden)
  last = start
  results = Set.new
  keys_variants.each do |keys|
    variants = [""]
    keys.split("").each do |digit|
      current = []
      from = mapping[last]
      to = mapping[digit]
      moves = [from[0]-to[0], from[1]-to[1]]
      if moves[0] < 0
        current += [">"] * moves[0].abs
      else
        current += ["<"] * moves[0].abs
      end
      if moves[1] < 0
        current += ["v"] * moves[1].abs
      else
        current += ["^"] * moves[1].abs
      end
      new_variants = current.permutation.uniq.select{|v| valid?(from, v, forbidden)} # all variants
      new_variants = new_variants.map{|v| v.join + "A"}
      variants = variants.map{|v| new_variants.map{|v_new| v + v_new}}.flatten
      last = digit
    end
    results += variants
  end
  results
end

# res = move("A",["456A"], @key_mapping_coordinates, [0,3])
# res = move("A",res, @arrow_mapping_coordinates, [0,0])
# res = move("A",res, @arrow_mapping_coordinates, [0,0])
# puts res.map{|r| r.size }.sort[0]

# puts res
# res = move("A",res, @arrow_mapping_coordinates, [0,0])
# puts res


result = 0
numbers.each do |number|
  res = move("A",[number], @key_mapping_coordinates, [0,3])
  res = move("A",res, @arrow_mapping_coordinates, [0,0])
  res = move("A",res, @arrow_mapping_coordinates, [0,0])
  #res = res.split("").map{|key| @arrow_mapping[key]}.join
  #puts "#{number}: #{res}"
  puts "#{res.map{|r| r.size }.sort[0]} * #{number[0..2].to_i}"
  result += res.map{|r| r.size }.sort[0] * number[0..2].to_i
end

puts "Part1: #{result}"