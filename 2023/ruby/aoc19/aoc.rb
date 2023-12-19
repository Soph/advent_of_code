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
parts = sections[1].split("\n").map do |line|
  items = {}
  line[1..-2].split(",").each do |item| 
    split = item.split("=")
    items[split[0]] = split[1].to_i
  end

  items
end

#puts workflows.inspect
#puts parts.inspect

accepted = []
parts.each do |part|
  workflow = workflows["in"]
  done = false
  while !done
    workflow.each do |instruction|
      #puts "#{part.inspect} -> #{workflow.inspect} -> #{instruction.inspect}"
      next_workflow = nil
      if instruction.size == 1
        next_workflow = instruction[0]
        #puts "next: #{next_workflow.inspect}"
      elsif part[instruction[0]]
        #puts "Checking #{instruction.inspect} on #{part.inspect}}"
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
          #puts "#{part.inspect} Accepted"
          accepted << part
          done = true
          break
        elsif next_workflow == 'R'
          #puts "#{part.inspect} Rejected"
          done = true
          break
        else
          #puts "New: #{next_workflow}"
          workflow = workflows[next_workflow]
          break
        end
      end
    end
    #sleep 1
  end
end

#puts accepted.inspect

#puts accepted.map{|a| a.values.sum}.inspect
puts accepted.map{|a| a.values.sum}.sum