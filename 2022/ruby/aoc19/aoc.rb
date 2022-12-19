require "set"
raw_blueprints = File.open(ARGV[0])
part = ARGV[1]

blueprints = Set.new
raw_blueprints.each do |raw_blueprint|
  data = raw_blueprint.match(/Blueprint (\d*): Each ore robot costs (\d*) ore. Each clay robot costs (\d*) ore. Each obsidian robot costs (\d*) ore and (\d*) clay. Each geode robot costs (\d*) ore and (\d*) obsidian./)

  blueprints << {
    id: data[1].to_i,
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
blueprints = blueprints.to_a[0..2] if part == "2"
blueprints.each do |blueprint|
  options = Set[{
    ore: 0,
    ore_robot: 1,
    clay: 0,
    clay_robot: 0,
    obsidian: 0,
    obsidian_robot: 0,
    geode: 0,
    geode_robot: 0,
    time: part == "2" ? 32 : 24
  }]
  done = Set.new
  max = 0
  i = 0
  cache = {}
  loop do
    option = options.take(1).first
    break if option.nil?
    options.delete(option)

    if max > 0 && (option[:geode] + option[:time] * option[:geode_robot] + option[:time] * (option[:time] - 1) / 2) < max
      next
    end    

    new_options = Set.new
    if option[:obsidian_robot] > 0 # produces obsidian -> can build geode robot now
      ore_needed = [0,(blueprint[:geode_robot][0] - option[:ore])].max
      obsidian_needed = [0,(blueprint[:geode_robot][1] - option[:obsidian])].max
      time = 1 # robot build time
      if ore_needed != 0 || obsidian_needed != 0
        time += [(ore_needed.to_f/option[:ore_robot]).ceil, (obsidian_needed.to_f/option[:obsidian_robot]).ceil].max
      end
      if option[:time] - time >= 1 # enough time
        opt = option.clone
        opt[:time] -= time
        update_produce(opt, time)
        opt[:geode_robot] += 1
        opt[:ore] -= blueprint[:geode_robot][0] # exising ore + new produced ore - cost of ore for robot
        opt[:obsidian] -= blueprint[:geode_robot][1] # exising ore + new produced ore - cost of ore for robot
        unless done.include?(opt)
          new_options << opt
        end
      end
    end

    if option[:clay_robot] > 0 # produces clay -> can build obsidian robot now
      ore_needed = [0,(blueprint[:obsidian_robot][0] - option[:ore])].max
      clay_needed = [0,(blueprint[:obsidian_robot][1] - option[:clay])].max
      time = 1 # robot build time
      if ore_needed != 0 || clay_needed != 0
        time += [(ore_needed.to_f/option[:ore_robot]).ceil, (clay_needed.to_f/option[:clay_robot]).ceil].max
      end

      if option[:time] - time >= 3 # at least time to produce an obsidian, build a geode bot, and let it produce one thing
        opt = option.clone
        opt[:time] -= time
        update_produce(opt, time)
        opt[:obsidian_robot] += 1
        opt[:ore] -= blueprint[:obsidian_robot][0] # exising ore + new produced ore - cost of ore for robot
        opt[:clay] -= blueprint[:obsidian_robot][1] # exising ore + new produced ore - cost of ore for robot
        unless done.include?(opt)
          new_options << opt
        end
      end
    end
    # build another clay robot
    ore_needed = [blueprint[:clay_robot][0] - option[:ore], 0].max
    time = 1 # bot build time
    if ore_needed > 0 
      time += (ore_needed.to_f/option[:ore_robot]).ceil
    end
    if option[:time] - time >= 5 # produce a clay, build a obsidian bot, let it produce an obsidian, build a geode bot and let it produce one thing
      opt = option.clone
      opt[:time] -= time
      update_produce(opt, time)
      opt[:clay_robot] += 1
      opt[:ore] -= blueprint[:clay_robot][0] # exising ore + new produced ore - cost of ore for robot
      unless done.include?(opt)
        new_options << opt
      end
    end
    # build another ore robot
    ore_needed = [blueprint[:ore_robot][0] - option[:ore], 0].max
    time = 1 # bot build time
    if ore_needed > 0 
      time += (ore_needed.to_f/option[:ore_robot]).ceil
    end
    if option[:time] - time >= 3 # at least time to produce an ore, build a geode bot, let it produce one geode
      opt = option.clone
      opt[:time] -= time
      update_produce(opt, time)
      opt[:ore_robot] += 1
      opt[:ore] -= blueprint[:ore_robot][0] # exising ore + new produced ore - cost of ore for robot
      unless done.include?(opt)
        new_options << opt
      end
    end

    ## done building
    if new_options.empty?
      done << option
    else
      new_max = new_options.map{|d| d[:geode] + (d[:geode_robot] * d[:time])}.max
      if new_max && new_max > max
        max = new_max
      end
      new_options.each do |option|        
        cache_key = option.values[0..-2]
        if cache[cache_key] && cache[cache_key] < option[:time]
          next
        end
        cache[cache_key] = option[:time]
        options << option
      end
    end
    i += 1
    puts "#{blueprint[:id]} done: #{done.size} options: #{options.size} max:#{max}" if i % 10000 == 0
  end
  done += options
  puts done.sort_by{|d| d[:geode] + (d[:geode_robot] * d[:time])}.last
  max_geode[blueprint[:id]] = done.map{|d| d[:geode] + (d[:geode_robot] * d[:time])}.max
  puts "Blueprint #{blueprint[:id]} done"
end
if part == "1"
  puts "Result 1 (Quality Levels): #{max_geode.map{|key,value| key * value}.sum}"
else
  puts "Result 2: #{max_geode.values.inject(:*)}"
end
