create table if not exists ele(
    id integer primary key autoincrement,
    number text,
    name text,
    catlog text,
    class text,
    source text,
    describe text,
	detail text,
	size text,
	regno text,
	researcher text,
	seqinfo text,
    sdate date);

    insert into ele (number,name,catlog,class,source,describe,detail,size,regno,researcher,seqinfo,sdate) values 
('2018YFA0901900-AspTer-TA','AspTer-TA','转氨酶','发现的元件','基因合成','Aspergillus terreus','转氨，密码子优化','987','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-AspOry-TA','AspOry-TA','转氨酶','发现的元件','基因合成','Aspergillus oryzae','转氨，密码子优化','990','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-MOTA ','MOTA ','转氨酶','发现的元件','基因合成','Mycolicibacterium obuense来源转氨酶','转氨，密码子优化','921','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-ATA117-rd11','ATA117-rd11','转氨酶','发现的元件','基因合成','Arthrobacter sp.来源的转氨酶','转氨，密码子优化','1005','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-MyLTA','MyLTA','转氨酶','发现的元件','基因合成','Mycolicibacterium litorale来源的转氨酶','转氨，密码子优化','1032','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-DCRJ55-TA','DCRJ55-TA','转氨酶','发现的元件','基因合成','Drechmeria coniospora来源的转氨酶 (hypothetical protein RJ55_07049)','转氨，密码子优化','999','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-PDC-E473Q(Zymomonas mobilis)','PDC-E473Q(Zymomonas mobilis)','丙酮酸脱羧酶','发现的元件','基因合成','Zymomonas mobilis来源的转氨酶','丙酮酸脱羧，密码子优化','1719','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-YPDC1-EQ','YPDC1-EQ','丙酮酸脱羧酶','发现的元件','基因合成','酵母','丙酮酸脱羧，密码子优化','1701','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-MULTISPECIES-aminotransferase IV(Actinobacteria)','MULTISPECIES-aminotransferase IV(Actinobacteria)','转氨酶','发现的元件','基因合成','Actinobacteria ','转氨，密码子优化','1038','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-aminotransferase IV(Mycobacterium sp.ACS1612)','aminotransferase IV(Mycobacterium sp.ACS1612)','转氨酶','发现的元件','基因合成','Mycobacterium sp.','转氨，密码子优化','1029','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-hypothetical protein UA08_09122(Talaromyces atroroseus)','hypothetical protein UA08_09122(Talaromyces atroroseus)','转氨酶','发现的元件','基因合成','Talaromyces atroroseus','转氨，密码子优化','972','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-AspFum','AspFum','转氨酶','发现的元件','基因合成','Aspergillus fumigatus','转氨，密码子优化','981','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-NeoFis','NeoFis','转氨酶','发现的元件','基因合成','Neosartorya fisheri ','转氨，密码子优化','982','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-TM1040','TM1040','转氨酶','发现的元件','基因合成','Ruegeria sp. TM1040 ','转氨，密码子优化','1380','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-omega-TA JS19666','omega-TA JS19666','转氨酶','发现的元件','基因合成','Polaromonas sp. JS666','转氨，密码子优化','1350','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-Vibrio fluvialis JS17 TA','Vibrio fluvialis JS17 TA','转氨酶','发现的元件','基因合成','Vibrio fluvialis','转氨，密码子优化','1371','None','上海交通大学林双君课题组','在TXT文档中','2022.01.16'),
('2018YFA0901900-Pyr1','Pyr1','FkbH like protein','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9','吡啶霉素中的生物合成元件，To synthesise glyceryl-S-acylcarrier protein ','1035','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr2','Pyr2','3-oxoacyl-[acyl-carrier protein] reductase ','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9','吡啶霉素中的生物合成元件，Catalyzes NADPH-dependent reduction of beta-ketoacyl-ACP intermediates. ','741','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr3','Pyr3','membrane protein','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除验证','吡啶霉素中的生物合成元件，Expose surfaces that are ideally suited for incorporation into, or binding to, membranes. ','1005','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr4','Pyr4','decaheme cytochrome c','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除验证','吡啶霉素中的生物合成元件，Foreign electron transporter','714','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr5','Pyr5','4-oxalocrotonate tautomerase','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除验证','吡啶霉素中的生物合成元件，Catalyzes the isomerization of beta,gamma-unsaturated enones to their alpha,beta-isomers','195','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr6','Pyr6','acyl-CoA dehydrogenase domain-containing protein','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除和生化实验验证','吡啶霉素中的生物合成元件。Accelerates the removal of hydrogen from metabolites ','1137','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr7','Pyr7','acyl carrier protein','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除和生化实验验证','吡啶霉素中的生物合成元件。Transfer the lipoyl group from one enzyme reaction to another','252','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr8','Pyr8','3-hydroxyacyl-CoA dehydrogenase, NAD-binding','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除和生化实验验证','吡啶霉素中的生物合成元件。Transfer two hydrogen atoms from organic compounds to electron acceptors','939','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr9','Pyr9','PKS','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除验证','吡啶霉素中的生物合成元件。Multifunctional enzyme that synthesizes a variety of polyketone compounds','5613','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr10','Pyr10','nicotinate-nucleotide pyrophosphorylase ','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除和生化实验验证','吡啶霉素中的生物合成元件。1.基因功能：催化喹啉酸和5-磷酸核糖-1-焦磷酸反应，一次生成烟酸单核糖磷酸，烟酸核糖以及烟酸2.构建体系：构建在pET28a的NdeI位点和EcoRI之间，在BL21宿主中进行表达。3.表达条件：菌株提前在LA(50ug/ml kan)培养基上划线培养，挑单克隆至LB(50ug/ml kan)中培养过夜至饱和，按1%接种量转接至大量LB(50ug/ml kan)中培养，用终浓度0.2mM的IPTG诱导蛋白表达，16℃表达20hr，收集菌体进行蛋白纯化。','927','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr11','Pyr11','L-aspartate oxidase ','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除验证','吡啶霉素中的生物合成元件。Catalyses the first step in the de novo biosynthesis of NAD+ ','1626','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr12','Pyr12','quinolinate synthase','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除验证','吡啶霉素中的生物合成元件。Catalyzes a unique condensation reaction between dihydroxyacetone phosphate and iminoaspartate','1047','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr13','Pyr13','PCP','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C9，经基因敲除验证','吡啶霉素中的生物合成元件。Carries peptide chains to perform enzymatic reactions such as condensation, reduction and dehydrogenation','270','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr14','Pyr14','A domain','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C21，经基因敲除验证','吡啶霉素中的生物合成元件 ','1713','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-Pyr15','Pyr15','4-phosphopantetheinyl transferase','发现的元件','基因组扩增','来源于Streptomyces B2517或Cosmid 20C22，经基因敲除验证','吡啶霉素中的生物合成元件 ','879','None','上海交通大学林双君课题组','None','2022.01.16'),
('2018YFA0901900-stnG4','stnG4','药物抗性转运蛋白/MFS家族转运蛋白','发现的元件','基因组扩增','来源于柔毛链霉菌，经基因敲除验证','负责调控链黑菌素生物合成','1497','None','上海交通大学林双君课题组','ctacacccggttcttgccgcggagggcggacaggatcgccgcgagtgccatgatccccatcagggtgtagaaggccgcgctgagtccgtcggcgaaggcgaccgcgagccggccctccagcttgctggtgccgacgaagatggcgaacgccaggtccctcgggatcgagtgcgaggccacgagcatcgccgtggtgaacgagaagaccatgccgatgctggagaacgtgcgcagcaccccggaggcgatacccaggcggtcgggcggtgacgccttcatcactgcggtgctgttggccgggaagaagaagctgccgccgatcatgatgatgagattgccgatcgcggcgagccacaggctgctggtcgccgtcatctgcgcgaagagcaccatggcgacgaccgagatgccgagccccagcgtggcggggatgaccgagccgaaccggtcggtgagccgcccggcatagggtcccaccaccgcgccgatcacatagccgggcaccagcagcagcgacgcgtcgatcgggtccatcccgcgtgtgccctggagatacatgatcaccaggaagagcaccgcgaagctgcccacgccctgcaacaaagcggccagcagcgagggcgccatcgtgggaatgcggaacagcgacaggtcgagtgtcggttccttctggtacagctcgatgacgacgaaggccacgagcagtgcgacaccgcccgccaggaatccgatcatggtcgcgtccagcgggcgggacgcgagcccggtcatcgcccacagcaccccgaagaggccgagcccgagcgtgatcatgccggcgaagtcgaacttcctgcggtgccgctcgctgcggtcccgcagcacacgtgccgccagcaccagcgcgacgacgccgatgggcacgttgatccagaagatccactgccacgacacataggtcaccatgacgccgccgagcaggaccccgagcacggagccgaacgagaagccgaaggcgttgaagccataggcgcgaccgcgctcctccggcggatagagctccgagatgatcgcgccgctgttggccgcgatcagcgccgcacccacgccctggagcagccggaacccgatgatgctgggcgcgttccaggcgagtgcgcacagcaccgagccgatcgtgaacaccaggaagccggctttgtacatccgcacccggccgaacatgtcgccgaacctgcccaattgcgtggagagcaccgtgaccacgaggagatagccgacgaccacccacgtgacggcggacagggtgatgttcaggtcttcttcgatctccggcagtgcgagcaccacgatcgtgccgtcgaccgccgagatgagcacaccgatcatcacgacgagcatgcccagagcgcgccgcggcggtgcggacggcgccggagagtccgccgtccccgtcacggtttcttcgctggccatcggtaaccccgcatccttccggtccggacttcttctgtcatgcaaggatgctagcat','2022.01.18'),
('2018YFA0901900-stnK4','stnK4','醛酮变位酶/博来霉素抗性基因/双加氧家族蛋白','发现的元件','基因组扩增','来源于柔毛链霉菌，经基因敲除验证','负责调控链黑菌素生物合成','423','None','上海交通大学林双君课题组','tcagctcttgagcggcgcctcgatgctccactcacgcccgtccgggtcgcggacgaccatccgctgggtgccgtagtgggtctcctcgaacggggagaccacctcgaccacggggtcgggcttgaaggtgtccgcgtccgcgacccgcagcacgagctggacctgcggctggcgctcctgggggatctcgccgacgacgaaatacggaccgtcgtcgtgcttgagcatccccgagccgtgctcggtggagaactccaccttgaagcccaacgactccaggaatttggccgtcttcccccagctgtgggtctccatgtacacgccttcgacaccctgggtcgtcatgtccgatgtcctctcactgaatggacttgtctcgttcttgccgctggtgctcgtccagataagaacgcagcgcctcggcaa','2022.01.18'),
('2018YFA0901900-stnE','stnE','DxoX家族蛋白','发现的元件','基因组扩增','来源于柔毛链霉菌，经基因敲除验证','负责调控链黑菌素生物合成','594','None','上海交通大学林双君课题组','gtgtgcgagtggggaaactccccactcgcacaccgacatcaccggaaaccggaagctaggttgctagtatcggttctgcgggccggcgccggggtggccggcgtacacgcggacgcctccgcccggcatggccggaaagtccgaggcggaccggtcgtcggggcccgtacggacgccggaggcgaaggaagggcagcgatggcggacacgggccggttgatcatcagaggcgtcgtcggcgggctggccatcggccgcggcggccggaagctgttcggctggttcggcgggccgggccgtgaccggaccgcccggaccttcgagtccttcggctaccgccacagccgggtgatggccaccgtggccgggatcaccgacctgagcaccggcgtgggactcctcgccgggctggccacccccctggccgcctccgccttcatcggcgagatggtcaacgccgcggcagtgcaccgacgagagggcccgtgggccgaggacggcggctacgagtatccgctggtgctcgccacagccgcggcctgtctcgcctgggacggcccgggcgccctgtccgccgaccgcgcgctcggacgcgagcggacaggccccgcctgggggctcggcgccatggccctgggcgcggccggcggcgtggccgtgctgcgtctcttccgcgacgagccggaatga','2022.01.18'),
('2018YFA0901900-pyr16','pyr16','赖氨酸脱羧酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','lysine decarboxylase','555','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr17','pyr17','硫酯酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','thioesterase','765','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr20','pyr20','P450羟化酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','P450 hydroxylase','1236','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr21','pyr21','甘油酰基转移酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','glycerol acyltransferase','687','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr22','pyr22','C5-甾醇去饱和酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','membrane-bound C-5 sterol desaturase','894','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr23','pyr23','磷酸烯醇式丙酮酸变位酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','phosphoenolpyruvate mutase','942','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr24','pyr24','磷酸烯醇式丙酮酸脱羧酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','phosphonopyruvate decarboxylase','1179','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr25','pyr25','氨基转移酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇',' aminotransferase ','1158','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr26','pyr26','氧化还原酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','FAD(NAD)-dependent oxidoreductase','2232','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr28','pyr28','糖苷水解酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','glycoside hydrolase','1506','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr29','pyr29','酯酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','esterase','1353','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr30','pyr30','纤维素酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','cellulase','1335','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr32','pyr32','氧化还原酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','oxidoreductase','903','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr33','pyr33','氨基水解酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','amidohydrolase','873','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr34','pyr34','CoA ligase编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','long-chain-fatty-acid--CoA ligase','1494','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr35','pyr35','烯酰基-CoA水合酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','enoyl-CoA hydratase','456','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr36','pyr36','氨基水解酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','amidohydrolase ','1380','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr37','pyr37','酰基-CoA脱氢酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','acyl-CoA dehydrogenase','1140','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr38','pyr38','3-甲基衣康酸异构酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','3-methylitaconate isomerase','1038','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-pyr39','pyr39','天冬酰胺合成酶编码基因','发现的元件','NRRL B-2517基因组','cosmid20C9基因簇','asparagine synthetase','1830','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-copB','copB','PKS','发现的元件','基因组扩增','Streptomyces sp.SNU607的15号基因簇，一种PKS','聚酮化合物合成酶,敲除无法产生coprisidins，回补后重新产生coprisidins','1269','None','上海交通大学林双君课题组','None','None'),
('2018YFA0901900-copC','copC','PKS','发现的元件','基因组扩增','Streptomyces sp.SNU607的15号基因簇，一种PKS','聚酮化合物合成酶,敲除无法产生coprisidins，回补后重新产生coprisidins','1272','None','上海交通大学林双君课题组','None','None'),
('2018YFA0901900-copA','copA','ACP','发现的元件','基因组扩增','Streptomyces sp.SNU607的15号基因簇，一种PKS','聚酮化合物合成酶,敲除无法产生coprisidins，回补后重新产生coprisidins','None','None','None','None','None'),
('2018YFA0901900-AKR3678','AKR3678','醛酮还原酶','发现的元件','基因组扩增、点突变','泰乐菌素产生菌','底物广谱性醛还原酶，表达条件为BL21(DE3)-pGro7-pET28a+,LB培养基，18℃ 18h','996','None','None','None','None'),
('2018YFA0901900-tylAI','tylAI','葡萄糖-1-磷酸胸苷转移酶','发现的元件','基因组扩增','泰乐菌素基因簇','葡萄糖-1-磷酸胸苷转移酶，BL21(DE3)-pET28a+,LB培养基，30℃ 6h','918','None','None','None','None'),
('2018YFA0901900-tylAII','tylAII','dTDP-葡萄糖4,6-脱水酶','发现的元件','基因组扩增','泰乐菌素基因簇','dTDP-葡萄糖4,6-脱水酶，BL21(DE3)-pET28a+,LB培养基，30℃ 7h','1002','None','None','None','None'),
('2018YFA0901900-tylJ','tylJ','dTDP-4-脱氢鼠李糖3,5-差向异构酶','发现的元件','基因组扩增','泰乐菌素基因簇','dTDP-4-脱氢鼠李糖3,5-差向异构酶，BL21(DE3)-pET28a+,LB培养基，30℃ 8h','618','None','None','None','None'),
('2018YFA0901900-tylD','tylD','NAD(P)依赖性氧化还原酶','发现的元件','基因组扩增','泰乐菌素基因簇','NAD(P)依赖性氧化还原酶，BL21(DE3)-pET28a+,LB培养基，30℃ 9h','1011','None','None','None','None'),
('2018YFA0901900-tylE','tylE','I类SAM依赖性甲基转移酶','发现的元件','基因组扩增','泰乐菌素基因簇','泰乐菌素糖基单元mycinose生物合成倒数第二步氧甲基转移酶，BL21(DE3)-pET28a+,LB培养基，30℃ 10h','1188','None','None','None','None'),
('2018YFA0901900-tylF','tylF','I类SAM依赖性甲基转移酶','发现的元件','基因组扩增','泰乐菌素基因簇','泰乐菌素糖基单元mycinose生物合成倒数第一步氧甲基转移酶，BL21(DE3)-pET28a+,LB培养基，30℃ 11h','771','None','None','None','None'),
('2018YFA0901900-CacoAT_YH','CacoAT_YH','转氨酶','创制的元件','改造','基于密码子优化的caco AT-FI构建的点突变','R转氨酶，邻位S选择性提高','1008','None','上海交通大学林双君组','None','2022.01.16'),
('2018YFA0901900-CacoAT_FI','CacoAT_FI','转氨酶','创制的元件','改造','基于密码子优化的caco AT-FI构建的点突变','R转氨酶，邻位R选择性提高','1008','None','上海交通大学林双君组','None','2022.01.16'),
('2018YFA0901900-CacoAT_FV','CacoAT_FV','转氨酶','创制的元件','改造','基于密码子优化的caco AT-FI构建的点突变','R转氨酶，邻位R选择性提高','1008','None','上海交通大学林双君组','None','2022.01.16'),
('2018YFA0901900-CacoAT_VA','CacoAT_VA','转氨酶','创制的元件','改造','基于密码子优化的caco AT-FI构建的点突变','R转氨酶，邻位R选择性提高','1008','None','上海交通大学林双君组','None','2022.01.16'),
('2018YFA0901900-CacoAT_WH','CacoAT_WH','转氨酶','创制的元件','改造','基于密码子优化的caco AT-FI构建的点突变','R转氨酶，邻位R选择性提高','1008','None','上海交通大学林双君组','None','2022.01.16'),
('2018YFA0901900-CacoAT_FIVF','CacoAT_FIVF','转氨酶','创制的元件','改造','基于密码子优化的caco AT-FI构建的点突变','R转氨酶，邻位R选择性提高','1008','None','上海交通大学林双君组','None','2022.01.16'),
('2018YFA0901900-CacoAT_FVVF','CacoAT_FVVF','转氨酶','创制的元件','改造','基于密码子优化的caco AT-FI构建的点突变','R转氨酶，邻位R选择性提高','1008','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-Pstn109','Pstn109','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1223 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','109','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn120','Pstn120','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1224 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','120','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn140','Pstn140','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1225 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','140','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn160','Pstn160','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1226 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','160','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn180','Pstn180','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1227 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','180','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn200','Pstn200','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1228 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','200','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn220','Pstn220','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1229 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','220','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn239','Pstn239','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1230 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','239','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-Pstn268','Pstn268','启动子','创制的元件','基因组扩增','S. flocculus CGMCC 4.1231 （Streptonigrin）','链黑菌素生物合成途径中优化的调控元件','268','None','上海交通大学林双君课题组','None','20220106'),
('2018YFA0901900-FonnuAT','FonnuAT','转氨酶','创制的元件','合成','Fonsecaea nubica','R转氨酶，大肠杆菌密码子偏好性优化','1029','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-FonmultiAT','FonmultiAT','转氨酶','创制的元件','合成','Fonsecaea multimorphosa CBS 102226','R转氨酶，大肠杆菌密码子偏好性优化','1014','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-ExxeAT','ExxeAT','转氨酶','创制的元件','合成','Exophiala xenobiotica','R转氨酶，大肠杆菌密码子偏好性优化','1008','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-DiciAT','DiciAT','转氨酶','创制的元件','合成','Diaporthe citri','R转氨酶，大肠杆菌密码子偏好性优化','969','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-AspcaAT','AspcaAT','转氨酶','创制的元件','合成','Aspergillus caelatus','R转氨酶，大肠杆菌密码子偏好性优化','1002','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-CacoAT','CacoAT','转氨酶','创制的元件','合成','Capronia coronata CBS 617.96','R转氨酶，大肠杆菌密码子偏好性优化','1008','None','上海交通大学林双君组','None','None'),
('2018YFA0901900-AKR3678','AKR3678','醛酮还原酶','创制的元件','基因组扩增、点突变','泰乐菌素产生菌','底物广谱性醛还原酶，表达条件为BL21(DE3)-pGro7-pET28a+,LB培养基，18℃ 18h','996','None','None','None','None');