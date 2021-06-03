# Fuse(P) / P
reg1 = /
\[WithOUT\ comp.\].*?\#MemAcc\ =\ (\d*),\ \#\[Fusioned\]MemAcc\ =\ (\d*)
/xm

# Fuse(Comp(P)) / Comp(P)
reg2 = /
\[With\ comp\.\].*?\#MemAcc\ =\ (\d*),\ \#\[Fusioned\]MemAcc\ =\ (\d*)
/xm

reg3 = /
\[WithOUT\ comp\.\].*?\#\[NoFusion\]Variables\ =\ (\d*).*?
\[With\ comp\.\].*?\#\[NoFusion\]Variables\ =\ (\d*)
\
/xm

reg3_ = /
\[WithOUT\ comp\.\].*?\#\[NoFusion\]Capacity\ =\ (\d*).*?
\[With\ comp\.\].*?\#\[NoFusion\]Capacity\ =\ (\d*)
\
/xm

reg4 = /
\[With\ comp\.\].*?\#\[NoFusion\]Variables\ =\ (\d*),\ \#\[Fusioned\]Variables\ =\ (\d*)
/xm

reg4_ = /
\[With\ comp\.\].*?\#\[NoFusion\]Capacity\ =\ (\d*),\ \#\[Fusioned\]Capacity\ =\ (\d*)
/xm


reg5 = /
\[With\ comp\.\].*?\#\[NoFusion\]Variables\ =\ (\d*).*?\#\[Fusioned\&Scheduled\]Variables\ =\ (\d*)
/xm

reg5_ = /
\[With\ comp\.\].*?\#\[NoFusion\]Capacity\ =\ (\d*).*?\#\[Fusioned\&Scheduled\]Capacity\ =\ (\d*)
/xm


reg6 = /
\[WithOUT\ comp\.\].*?\#\[NoFusion\]Variables\ =\ (\d*).*?\#\[Fusioned\]Variables\ =\ (\d*)
\
/xm

reg6_ = /
\[WithOUT\ comp\.\].*?\#\[NoFusion\]Capacity\ =\ (\d*).*?\#\[Fusioned\]Capacity\ =\ (\d*)
\
/xm

# Fuse(Comp(P)) / P
reg7 = /
\[WithOUT\ comp.\].*?\#MemAcc\ =\ (\d*).*?
\[With\ comp\.\].*?\#\[Fusioned\]MemAcc\ =\ (\d*)
/xm

# Comp(P) / P
reg8 = /
\[WithOUT\ comp.\].*?\#MemAcc\ =\ (\d*).*?
\[With\ comp\.\].*?\#MemAcc\ =\ (\d*)
/xm

reg9 = /
Dec\ (.*?)\:.*?\[WithOUT\ comp.\]\ \#XOR\ =\ (\d*),
/xm

File.open("all_stat.txt") {|f|
  s = f.read
  r = s.scan(reg9)

  tuples = r.map { |x|
    [x[0], x[1].to_i]
    # x.map(&:to_i)
  }

  p tuples.sort{|a, b| a[1] <=> b[1]}

#  reduced = tuples.map { |x|
#    v = x[1].to_f / x[0].to_f
#    puts "#{x[1]}, #{x[0]}, #{v}"
#    v
#  }
#  p reduced.sum(0.0) / reduced.size
}
