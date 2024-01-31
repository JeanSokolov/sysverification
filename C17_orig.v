// IWLS benchmark module "C17.iscas" printed on Wed Mar  6 23:40:18 2002
module \C17.iscas (\1GAT(0) , \2GAT(1) , \3GAT(2) , \6GAT(3) , \7GAT(4) , \22GAT(10) , \23GAT(9) );
input  \2GAT(1) ;
input  \7GAT(4) ;
input  \3GAT(2) ;
input  \1GAT(0) ;
input  \6GAT(3) ;
output \22GAT(10) ;
output \23GAT(9) ;
wire   \19GAT(7) ;
wire   \10GAT(6) ;
wire   \11GAT(5) ;
wire   \[0] ;
wire   \16GAT(8) ;
wire   \[1] ;
  assign \22GAT(10)  = \[0] ;
  assign \19GAT(7)  = ~\7GAT(4)  | ~\11GAT(5) ;
  assign \23GAT(9)  = \[1] ;
  assign \10GAT(6)  = ~\3GAT(2)  | ~\1GAT(0) ;
  assign \11GAT(5)  = ~\6GAT(3)  | ~\3GAT(2) ;
  assign \[0]  = ~\16GAT(8)  | ~\10GAT(6) ;
  assign \16GAT(8)  = ~\11GAT(5)  | ~\2GAT(1) ;
  assign \[1]  = ~\19GAT(7)  | ~\16GAT(8) ;
end;

