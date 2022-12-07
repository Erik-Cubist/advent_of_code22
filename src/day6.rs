use substring::Substring;
use itertools::Itertools;

fn solve1(data: String) -> usize {
    solve(data, 4)
}

fn solve(data: String, size : usize) -> usize {
    for i in 0.. {
        if data.substring(i, i + size).chars().unique().count() == size {
            return i + size
        }
    }
    panic!("never reached");
}        
 
 fn solve2(data: String) -> usize {
     solve(data, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        let result = solve1(input_example1());
        assert_eq!(result, 7);
    }

    #[test]
    fn example1b() {
        let result = solve1(input_example2());
        assert_eq!(result, 5);
    }

    #[test]
    fn example1c() {
        let result = solve1(input_example3());
        assert_eq!(result, 6);
    }

    #[test]
    fn example1d() {
        let result = solve1(input_example4());
        assert_eq!(result, 10);
    }

    #[test]
    fn example1e() {
        let result = solve1(input_example5());
        assert_eq!(result, 11);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise());
        assert_eq!(result, 1707);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example1());
        assert_eq!(result, 19);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise());
        assert_eq!(result, 3697);
    }

    fn input_example1() -> String {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()
    }

    fn input_example2() -> String {
        "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()
    }

    fn input_example3() -> String {
        "nppdvjthqldpwncqszvftbrmjlhg".to_string()
    }

    fn input_example4() -> String {
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()
    }

    fn input_example5() -> String {
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()
    }

    fn input_exercise() -> String {
        "rqwqtwwrmrqqrcrtrpppjrjgrgjgqgttgbbhppftpfpdpmpsstffqvfqvvdjjpllltqltqtrrzlllbldbdsdqqgpqggzccgbgnnmjmljmljldjldjjlnnhndhdmmslmslmlhmmntnrtnrnbrbqrqhqfftwftfsfzfcfrrzllchcwwtwstwwjwrjjfwwgzwzqzpzhzqhqbhqhssltsszhsszdssrwssdppjcpjjrwjwnwjnnzjjhsjhsjsjvvtffffqfflglwglgbgpbgbgghfghfhdfhdfhfssqppvlvslltwwhwvwpwgggnpnnmqnntqnnlhhdbhdbbtvvcrrcgrrdrvrjvvlmlnmndnmnrncnznsnsqnsswrwzwmwzzbpbhphqhpqpbpqpjpjvpvwpvwppdvdhdccqbccfncctrcrjcjjdsdhdnhhwpwlwwbmbhmbhhpzhhccvffqqnsqqmnnspnnpfpzppspgsppgbbdwbbdcchfhgfhfllljtllnjlnlddvcdvdqvvqwqppprtpthtthqqfbqqpfpzfpfcfsscrsccbgbngbbgdbbpgpqqlmqlmmgffqqqsbqbtblbwbqqvbqvbqvvnzvzpvvqhvqqtltptllbflbflbflltddlblflvfvbffmsffhlhshmmgbbgjjdndqqmqllzjzvvhjhnhcnchhznnhhppwqpwpffwvwzzppwpfwftwtnnqbnqnrnrzznffzzblzlvzlzrrszscsqcssbggcgvvddsttgqqgwgcclzzszddlvddvbddrlrcllrblrrhlldzdtdccqllsblsszlszzdllhjljhhhzhffdwfdwfddwdtdrtdtntddldsllqffdvdhdpdhdrdprprrhggdqgghggmwgmghhchlhmllrnrdrcrqccbqqfbqbccjpjhhvppcwpwffmmzzfqflfjfdfbfqqtmthmttwftwwpmpzplzzpvvgppsbpbccbtcbcddzbzwwmsscrcctgtgjgtjthhjzzcqqmmmvccwjcwjwzzwnnwgwtgwwbsbvbccrscccnggfjfwfjfrfsfddncdncndnqqwzzmssbdsbswswdddpdqdbddcvcmmwwssrtrrlddndmdtmtdmttwwmsmffwqwvwbwhbbqfqqgcqcmctctntjntnbnrbrmbbqtqccqbcbvbnbqnnghgbbmfmwmdwmdmpmwppdwpdwwnzwnwrnwrnntqnttcmcqqbrqrttsjtstcscpcspsnswsggrhggjrjjbhhbnbmmdlmddtqdtdrdwwjvjgjjdhjdddbwwsfflppbwbcbpbjjzqznnrwnnvmvjvwvqqrlljgjmjjqhjhwjhwjjsvsnvvlwlqqjpjjbsjjcddtfthffrqfrqrfqfrfllfzzqllztltnltntpttqmmgwmwwwjjdrjrzrlrwlwzzzzphpphvphvphvpvdvfdflljgglrlrqlrrfpfqpqnpqpmmblbflbfbpfpwpjwpjpbjpjmjljblldbllnrnjjjljrlrcrrgzgbbzrznzzhnzhhtmhmqhmqqnzzvfzvzfftjftfvvpmvppwzzhnzhzvvwzwwqcwctwwgrrtgglgngsgvgwvwpvpbbrvrvwrvwvbwwpsprrrcbbpqppfjjjqnjndnmddvsvlslmmwwdwcchmmrjmjwwsrwrhwwpcgmpmvbdmlmqbnblbbgtmtgqslnvcnmwrflrnwhqzrsnwhncwdvbcpjrqsscwzrjvslhscpvmqjnltwhshjmbhgttbzllrvpnhgdmwtddbwmpgbhrndpgwzfvqdmpcvhlnjnqbwqmshgwhmfjgsbscsjtfbfvmlljqwwjtnltbzjpcqdfmdldvmsnmzqmghcvhlpjzzszcpvldrflbcppzwgppfpbslplrqwgbfwpwbtbnrmbrrrpqndzvhmlhvtlnjvhdrhhvmwzjntsgffjsdpdlhnlslnbmbsmdsvpmpbjpjcwhbpfnhbmttfglglnnhhcqvbpvnrwcszwjsdhqwctrpgsrcbfpzhbnrbllrlflqdlgzwpwgwwvmbsppbcjcdltbgwfndqjlvndhtclfmwnqrbfvvqdzspfcslplhtmsqqtrcdvbhfscvnmzplcglwnzbvgflclclqmdgfzjqqrrmpcqrzgqcnzbmncmvdzrmmvfqrrqzqbjmjgjwmfbnmmgvmzsgjjspjqbvfhrdvllgqsffjnfqdthjbjwhwqdqhsnctwgbszvrsbwbpptcjjvftnhdmwmhtjrjrlrdrqjznhqftgtldnpbrlprjmplmwdbzqpmwwvcqccjlhqbhcjprvrfmmfmljwmbrlhcglthgbmrfmbpsztfcglpzlfbpjhmvcmvrprmvmrvvgbbllzschwshhfbnmgwrbhlqvsdjdmmbtjzthwssghtqvhtqcswdhbwhphhsrwhwwtslwvrhpgqfmftnrtqpwfqqdgftdlqfndjlzhvlrthnqdqrzpprdgwwqlplrmqtdqgbdntmjcjhlbbmctvnnhtppfbjpsbmndbdplqlfqfhzqvtqcvmprwbhdtjrqrqvmcssqnfqwtchmjfjlpwsghplbthdnbhfbhhphdmlmtjcvmzjbnqbfldrvlmjfnnrlsnwrtnrpfqdhchltmbvjzhgjwzblqdthmffztqvzzfszmdqdzlfpbfgjfdjqscrhvjflbllnlghncczrlpcbwpnmzcnqhbfnlmnbqvmwvbchwhlfcssctsdcwmtfdgbhljwprfncdgwlgzzmvfqrdsmlwbrmbmtzqqdqdpzvtbstscglrwdpndnstrhgjchnpzsvcjngmzsddwsbllpvqqzzjlmsbncbndqmqphzqzcngjzpfnjmsvngtcvhhjsssrnhjrsmvbsrdjcrppvgsqbqjzbrhjqlgmqfclbrdhwwjdlsjnmjcjrmsstngswrhmdqthprcbrndfpdjgpqtfmmfvbztqdhzhzbzhjjpfgnvrmspdvmhfvbvztmlrhfqfdcfnzzndlfldpfhrbjrwlwnprdmqjwppbrdjhpbvnrvjlrbwdwttzhqqgcclbwghbgrhcvwjrrchqgzztrjgrltcpdscjdfcphndfzdqsbpsjrljdnflfvrzcrjflddfgpvnpmdqbhzwbfrpzsnpbcbgfchvvmqpnfpvvdvhfnpbfzwmbnlpqzgrpwtsjbpzdrsfmnfqwqvpgnrjhmvmphlfdpfhdjljlzftdzdvhjrvjdctwfrctglwmrtdlrvmhcqvfvpgmstghczmvclptzspdsrlvrgvjtvflhsmqswdpqjrrnhgrggzmcpqsctqbhpqrsbstwzgzghqgltcpbrhbcdqlhfjhlcrnhrpjdzlzqqprvzntbjztqgdqqttmbpbcfbjmdjqsflbczmfnsjdzcnnmjjllhvwbwlgrlpvbffwjvctwnrsfqdnwntscpsjnshhdmcqmcpscrmzltldwlmfnbhbtprsgtsbvzfrsfpvmqjzjqqhwdjjzvbhrvtbscrcpzrqsbgfbpwqfmbsrlhhtntjmtrtlwqsdbmgtdlrwfdnmwwzlltbrjgghbstbnffzmzzdlcdgvmrzsjnqjvczmhrgwbrblrqfmzbbcnfsbfpgjsmptzbclfdnwfgbzlpbgqrdhdmfjdnlzczdpvtbzdmrthslngbrldllfcjplhglmwsfpqjnrgpgqfmbbwchbrqqsncfwnmgdmtchdhlzzqjvtvtvgqbtwvwpsjqwdqhrlfmdgdzchvfbcmqjllpjtnphwqvfwpcjmzqhsbwlcmbsgththrsjtnzlsrcptgtrfcwptbstcsdzbwjljzjztzbqsjdvwglpbpgdjtthjjmsnljltglqcszbzqmblfpncntjzzhmjffldcrcvjjswvzlfffrmsgjtpzggtlpfwpwbmhmggpnmzwrgdjrhglnhfdcjdfjtdjvrnlgqtfqmpgjcvcmnwhhbczwwntfbmgssglngqlttfpcznswvmbprsrzljtlwlljnbbrnwdvvlsbdv".to_string()
    }
}
