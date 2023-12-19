require "Set"
file = File.open(ARGV[0])
sections = file.read.split("\n\n")

workflows = {}
sections[0].split("\n").each do |workflow|
  # px{a<2006:qkq,m>2090:A,rfg}
  key = workflow.gsub(/\{.*\}/, "")
  workflows[key] = workflow.gsub(/.*\{/, "")[0..-2].split(",").map do |item|
    # a<2006:qkq
    a = []
    if item =~ /\:/
      a << item[0...1]
      a << item[1...2]
      a += item[2..].split(":")
      a[2] = a[2].to_i
    else
      a << item
    end
    a
  end
end

#puts workflows.inspect

paths = Set.new
paths << [{"in" => []}]

def invert(instruction)
  if instruction[1] == '>'
    [instruction[0], '<', instruction[2] + 1]
  else
    [instruction[0], '>', instruction[2] - 1]
  end
end

done = Set.new
while true
  new_paths = Set.new
  paths.each do |path|
    next if path.last.keys.first == 'R'
    if path.last.keys.first == 'A'
      done << path
      next
    end
    prior = []
    workflows[path.last.keys.first].each do |instruction|
      if instruction.size == 1
        new_paths << path + [{instruction[0] => prior}]
      else
        new_paths << path + [{instruction[3] => [instruction[0..2]] + prior}]
        prior << invert(instruction)
      end
    end
  end
  break if new_paths.empty?
  paths = new_paths
end

happy_paths = done.delete_if{|path| path.last.keys.first == 'R'}
# happy_paths.each do |d|
#   puts d.inspect
# end

conditions = []
ranges = { 
  "m" => (1..4000),
  "a" => (1..4000),
  "s" => (1..4000),
  "x" => (1..4000)
}
valid_ranges = []
happy_paths.each do |path|
  range = { 
    "m" => [1,4000],
    "a" => [1,4000],
    "s" => [1,4000],
    "x" => [1,4000]
  }
  path[1..].each do |step|
    step.values.each do |values|
      values.each do |value|
        if value[1] == '>'
          range[value[0]] = [[(value[2]+1),range[value[0]][0]].max,range[value[0]][1]]
        else
          range[value[0]] = [range[value[0]][0],[(value[2]-1), range[value[0]][1]].min]
        end
      end
    end
  end
  valid_ranges << range
end

results = []
# valid_ranges.each do |range| 
#   puts range.inspect
#   puts range.values.map{|v| (v[0]..v[1])}.map(&:size).inject(&:*)
# end
puts valid_ranges.map {|range| range.values.map{|v| (v[0]..v[1])}.map(&:size).inject(&:*) }.sum
exit