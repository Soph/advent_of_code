require "set"
raw_blueprints = File.open(ARGV[0])

blueprints = {}
raw_blueprints.each do |raw_blueprint|
  data = raw_blueprint.match(/Blueprint (\d*): Each ore robot costs (\d*) ore. Each clay robot costs (\d*) ore. Each obsidian robot costs (\d*) ore and (\d*) clay. Each geode robot costs (\d*) ore and (\d*) obsidian./)

  blueprints[data[1].to_i] = {
    ore_robot: [data[2].to_i],
    clay_robot: [data[3].to_i],
    obsidian_robot: [data[4].to_i, data[5].to_i],
    geode_robot: [data[6].to_i, data[7].to_i],
  }
end

def update_produce(variant, time)
  variant[:ore] += variant[:ore_robot] * time
  variant[:clay] += variant[:clay_robot] * time
  variant[:obsidian] += variant[:obsidian_robot] * time
  variant[:geode] += variant[:geode_robot] * time
end

max_geode = {}
blueprints.each do |id, blueprint|
  options = Set[{
    ore: 0,
    ore_robot: 1,
    clay: 0,
    clay_robot: 0,
    obsidian: 0,
    obsidian_robot: 0,
    geode: 0,
    geode_robot: 0,
    time: 24
  }]
  puts blueprint.inspect
  done = Set.new
  max = 0
  loop do
    #puts options.inspect
    #sleep 1
    option = options.take(1).first
    break if options.size == 0
    options.delete(option)

    if (option[:geode] + (option[:geode_robot] + 1) * option[:time]) <= max
      #throw away
      next
    end

    new_options = Set.new
    if option[:obsidian_robot] > 0 # produces obsidian -> can build geode robot now
      ore_needed = [0,(blueprint[:geode_robot][0] - option[:ore])].max
      obsidian_needed = [0,(blueprint[:geode_robot][1] - option[:obsidian])].max
      time = [(ore_needed.to_f/option[:ore_robot]).ceil, (obsidian_needed.to_f/option[:obsidian_robot]).ceil].max
      #puts "Geode: ore: #{blueprint[:geode_robot][0]}/#{ore_needed}, obsidian: #{blueprint[:geode_robot][1]}/#{obsidian_needed} -> time: #{time}"
      time += 1 # until ready
      if option[:time] - time > 0 # enough time
        opt = option.clone
        opt[:time] -= time
        update_produce(opt, time)
        opt[:geode_robot] += 1
        opt[:ore] -= blueprint[:geode_robot][0] # exising ore + new produced ore - cost of ore for robot
        opt[:obsidian] -= blueprint[:geode_robot][1] # exising ore + new produced ore - cost of ore for robot
        new_options << opt
        puts "Build Geode Robot. time:#{time} - #{option} -> #{opt}"
      end
    end

    if option[:clay_robot] > 0 # produces clay -> can build obsidian robot now
      ore_needed = [0,(blueprint[:obsidian_robot][0] - option[:ore])].max
      clay_needed = [0,(blueprint[:obsidian_robot][1] - option[:clay])].max
      time = 1
      if ore_needed != 0 || clay_needed != 0
        time += [(ore_needed.to_f/option[:ore_robot]).ceil, (clay_needed.to_f/option[:clay_robot]).ceil].max
      end
      #puts "Obsidian: ore: #{blueprint[:obsidian_robot][0]}/#{ore_needed}, obsidian: #{blueprint[:obsidian_robot][1]}/#{clay_needed} -> time: #{time}"

      if option[:time] - time > 0 # enough time
        opt = option.clone
        opt[:time] -= time
        update_produce(opt, time)
        opt[:obsidian_robot] += 1
        opt[:ore] -= blueprint[:obsidian_robot][0] # exising ore + new produced ore - cost of ore for robot
        opt[:clay] -= blueprint[:obsidian_robot][1] # exising ore + new produced ore - cost of ore for robot
        new_options << opt
      end
    end
    # build another clay robot
    time = ([blueprint[:clay_robot][0] - option[:ore], 0].max.to_f/option[:ore_robot]).ceil
    time += 1 # until ready
    if option[:time] - time > 0 # enough time
      opt = option.clone
      opt[:time] -= time
      update_produce(opt, time)
      opt[:clay_robot] += 1
      opt[:ore] -= blueprint[:clay_robot][0] # exising ore + new produced ore - cost of ore for robot

      new_options << opt
    end
    # build another ore robot
    time = ([blueprint[:ore_robot][0] - option[:ore],0].max.to_f/option[:ore_robot]).ceil
    time += 1 # until ready
    if option[:time] - time > 0 # enough time
      opt = option.clone
      opt[:time] -= time
      update_produce(opt, time)
      opt[:ore_robot] += 1
      opt[:ore] -= blueprint[:ore_robot][0] # exising ore + new produced ore - cost of ore for robot
      new_options << opt
    end

    ## done building
    found = false
    new_options.each do |new_option|
      next if done.include?(new_option)
      next if options.include?(new_option)
      options << new_option
      found = true
    end
    done << option if !found

    new_max = done.map{|d| d[:geode] + (d[:geode_robot] * d[:time])}.max
    if new_max && new_max > max
      max = new_max
    end
   
    puts "done: #{done.size} options: #{options.size} max:#{max}"
  end
  #puts done.sort_by{|d| d[:geode_robot]}
  #sleep 20
  max_geode[id] = (done + options).map{|d| d[:geode] + (d[:geode_robot] * d[:time])}.max
  puts "Next Blueprint"
  sleep 1
end
puts max_geode.inspect
