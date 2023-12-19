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
happy_paths.each do |d|
  puts d.inspect
end

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
        puts value.inspect
        if value[1] == '>'
          range[value[0]] = [value[2]+1,range[value[0]][1]]
        else
          range[value[0]] = [range[value[0]][0],value[2]-1]
        end
      end
    end
  end
  valid_ranges << range
end

results = []
valid_ranges.each do |range| 
  puts range.inspect
  puts range.values.map{|v| (v[0]..v[1])}.map(&:size).inject(&:*)
end
puts valid_ranges.map {|range| range.values.map{|v| (v[0]..v[1])}.map(&:size).inject(&:*) }.sum
exit

214095809228000
167409079868000
167409079868000

# [{"in"=>[]}, 
#  {"px"=>[["s", "<", 1351]]},
#  {"rfg"=>[["a", ">", 2005], ["m", "<", 2091]]}, 
#  {"A"=>[["s", ">", 536], ["x", "<", 2441]]}]

numbers = {}
happy_paths.each do |path|
  path.each do |workflow_name|
    workflow = workflows[workflow_name]

  end
  numbers[path.join("->")] = ranges
end

workflow = workflows["in"]



workflow.instructions.each do |instruction|
  #in{s<1351:px,qqz}
  s1 = (0...1351)
  s2 = (1351..4000)
end

variants = [
  { 
    "m" => (1..4000),
    "a" => (1..4000),
    "s" => (1..4000),
    "x" => (1..4000)
  }
]
accepted = []

workflow = workflows["in"]
while true
  workflow.instructions.each do |instruction|
    if instruction.size == 1 # goto

    else
      new_variants = []
      variant = variants.pop
      while true
        if instruction[1] == '>'
          if variant[instruction[0]].min > instruction[2]
            new_variants << variant

          else
            new_variant = variant.clone
            new_variant[instruction[0]] = (variant[instruction[0]].min...instruction[2])
            new_variants << new_variant
            new_variant = variant.clone
            new_variant[instruction[0]] = (instruction[2]...variant[instruction[0]].max)
            new_variants << new_variant
          end
        elsif instruction[1] == '<'

        end
      end
    end
  end
end

parts.each do |part|
  workflow = workflows["in"]
  done = false
  while !done
    workflow.each do |instruction|
      #puts "#{part.inspect} -> #{workflow.inspect} -> #{instruction.inspect}"
      next_workflow = nil
      if instruction.size == 1
        next_workflow = instruction[0]
        puts "next: #{next_workflow.inspect}"
      else
        puts "Checking #{instruction.inspect} on #{part.inspect}}"
        if instruction[1] == ">"
          if part[instruction[0]] > instruction[2]
            next_workflow = instruction[3]
          end
        elsif instruction[1] == "<"
          if part[instruction[0]] < instruction[2]
            next_workflow = instruction[3]
          end
        end
      end
      if next_workflow
        if next_workflow == 'A'
          puts "#{part.inspect} Accepted"
          accepted << part
          done = true
          break
        elsif next_workflow == 'R'
          puts "#{part.inspect} Rejected"
          done = true
          break
        else
          puts "New: #{next_workflow}"
          workflow = workflows[next_workflow]
          break
        end
      end
    end
    #sleep 1
  end
end

puts accepted.inspect

puts accepted.map{|a| a.values.sum}.inspect
puts accepted.map{|a| a.values.sum}.sum