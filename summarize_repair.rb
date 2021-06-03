reg = /
iter\ =\ (\d*).*?
original_slp.*?\ =\ (\d*).*?
defs_trivial.len.*?\ =\ (\d*).*?
defs_distance.len.*?\ =\ (\d*).*?
defs_long.len.*?\ =\ (\d*)
/xm

def fst(a, b, c)
  ary = [a, b, c].uniq.sort
  ary[0]
end

def snd(a, b, c)
  ary = [a, b, c].uniq.sort
  if ary.size >= 1
    ary[1]
  else
    nil
  end
end

def trd(a, b, c)
  ary = [a, b, c].uniq.sort
  if ary.size >= 2
    ary[2]
  else
    nil
  end
end

# solo leader
def top(a, b, c)
  ary = [a, b, c].sort
  if ary.size >= 2 and ary[0] < ary[1]
    ary[0]
  else
    nil
  end
end
   
def count(tuples, pos)
  score = { fst: 0, snd: 0, trd: 0, top: 0 }

  p tuples.size
  
  tuples.each { |tuple|
    _, a, b, c = tuple
    if tuple[pos] == fst(a, b, c)
      score[:fst] += 1
    elsif tuple[pos] == snd(a, b, c)
      score[:snd] += 1
    elsif tuple[pos] == trd(a, b, c)
      score[:trd] += 1
    end

    if tuple[pos] == top(a, b, c)
      score[:top] += 1
    end
  }

  score
end

File.open("log.log") {|f|
  s = f.read
  # puts s
  r = s.scan(reg)
  # p r

  tuples = r.map { |x|
    iter, orig, triv, dist, long = x
    [orig.to_i, triv.to_i, dist.to_i, long.to_i]
  }

  # p tuples

  # count_triv = { fst: 0, snd: 0, trd: 0 }
  # count_dist = [0, 0, 0]
  # count_long = [0, 0, 0]
  
  count_triv = count(tuples, 1)
  p count_triv
  
  count_dist = count(tuples, 2)
  p count_dist
  
  count_long = count(tuples, 3)
  p count_long
}

=begin
p fst(0, 1, 2)
p snd(0, 1, 2)
p trd(0, 1, 2)
p fst(0, 0, 0)
p snd(0, 0, 0)
p trd(0, 0, 0)
p fst(5, 0, 0)
p snd(5, 0, 0)
p trd(5, 0, 0)
p fst(5, 2, 0)
p snd(5, 2, 0)
p trd(5, 2, 0)
=end
